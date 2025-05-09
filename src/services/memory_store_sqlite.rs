use crate::services::memory_store::*;
use rusqlite::{params, named_params, Connection, Row};
use chrono::{DateTime, Utc};

pub struct SqliteMemoryStore {
    pub conn: Connection,
}

impl SqliteMemoryStore {
    pub fn new(db_path: &str) -> Result<Self, MemoryError> {
        let conn = Connection::open(db_path)
            .map_err(|e| MemoryError::Database(e.to_string()))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS memory (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id TEXT NOT NULL,
                memory_type TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                tags TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_memory_user_type ON memory(user_id, memory_type);"
        ).map_err(|e| MemoryError::Database(e.to_string()))?;
        Ok(Self { conn })
    }

    fn row_to_memory(row: &Row) -> rusqlite::Result<Memory> {
        let tags_str: String = row.get("tags")?;
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
        Ok(Memory {
            id: row.get("id")?,
            owner_id: row.get("user_id")?, // 與 user_id 欄位值相同以保留向後相容
            user_id: row.get("user_id")?,
            memory_type: row.get("memory_type")?,
            content: row.get("content")?,
            created_at: row.get::<_, String>("created_at")?.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now()),
            scope: row.get("scope").unwrap_or_else(|_| "ai-visible".to_string()),
            tags,
        })
    }
}

impl MemoryStore for SqliteMemoryStore {
    fn add_memory(&self, mem: &Memory) -> Result<i64, MemoryError> {
        let tags_json = serde_json::to_string(&mem.tags).map_err(|e| MemoryError::Serialize(e.to_string()))?;
        self.conn.execute(
            "INSERT INTO memory (user_id, memory_type, content, created_at, tags) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![&mem.user_id, &mem.memory_type, &mem.content, mem.created_at.to_rfc3339(), &tags_json]
        ).map_err(|e| MemoryError::Database(e.to_string()))?;
        Ok(self.conn.last_insert_rowid())
    }

    fn get_memories(&self, query: MemoryQuery) -> Result<Vec<Memory>, MemoryError> {
        let mut sql = String::from("SELECT * FROM memory WHERE 1=1");
        let mut params: Vec<(String, String)> = Vec::new();
        if let Some(ref uid) = query.user_id {
            sql.push_str(" AND user_id = :user_id");
            params.push((":user_id".into(), uid.clone()));
        }
        if let Some(ref mt) = query.memory_type {
            sql.push_str(" AND memory_type = :memory_type");
            params.push((":memory_type".into(), mt.clone()));
        }
        if let Some(since) = query.since {
            sql.push_str(" AND created_at >= :since");
            params.push((":since".into(), since.to_rfc3339()));
        }
        if let Some(until) = query.until {
            sql.push_str(" AND created_at <= :until");
            params.push((":until".into(), until.to_rfc3339()));
        }
        if let Some(limit) = query.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        let mut stmt = self.conn.prepare(&sql).map_err(|e| MemoryError::Database(e.to_string()))?;
        let rows = stmt.query_map(named_params!{
            ":user_id": &query.user_id.as_deref().unwrap_or(""),
            ":memory_type": &query.memory_type.as_deref().unwrap_or(""),
            ":since": &query.since.as_ref().map(|d| d.to_rfc3339()).unwrap_or_else(|| "".to_string()),
            ":until": &query.until.as_ref().map(|d| d.to_rfc3339()).unwrap_or_else(|| "".to_string())
        }, |row| SqliteMemoryStore::row_to_memory(row))
        .map_err(|e| MemoryError::Database(e.to_string()))?;
        let results: Result<Vec<_>, _> = rows.collect();
        let results = results.map_err(|e| MemoryError::Database(e.to_string()))?;
        Ok(results)
    }

    fn update_memory(&self, id: i64, new_content: &str, tags: Option<Vec<String>>) -> Result<(), MemoryError> {
        let tags_json = serde_json::to_string(&tags.unwrap_or_default()).map_err(|e| MemoryError::Serialize(e.to_string()))?;
        let n = self.conn.execute(
            "UPDATE memory SET content = ?1, tags = ?2 WHERE id = ?3",
            params![new_content, tags_json, id]
        ).map_err(|e| MemoryError::Database(e.to_string()))?;
        if n == 0 { return Err(MemoryError::NotFound); }
        Ok(())
    }

    fn delete_memory(&self, id: i64) -> Result<(), MemoryError> {
        let n = self.conn.execute(
            "DELETE FROM memory WHERE id = ?1",
            params![id]
        ).map_err(|e| MemoryError::Database(e.to_string()))?;
        if n == 0 { return Err(MemoryError::NotFound); }
        Ok(())
    }
}
