use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FixRecord {
    pub id: i64,
    pub timestamp: i64,
    pub err_sig: String,
    pub patch_sig: String,
    pub success: bool,
    pub meta: Option<String>,
}

pub struct KnowledgeBase {
    conn: Connection,
}

impl KnowledgeBase {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS fix_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                err_sig TEXT NOT NULL,
                patch_sig TEXT NOT NULL,
                success INTEGER NOT NULL,
                meta TEXT
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert(&self, record: &FixRecord) -> Result<()> {
        self.conn.execute(
            "INSERT INTO fix_records (timestamp, err_sig, patch_sig, success, meta) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                record.timestamp,
                record.err_sig,
                record.patch_sig,
                record.success as i32,
                record.meta
            ],
        )?;
        Ok(())
    }

    pub fn query(&self, err_sig: &str) -> Result<Vec<FixRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, err_sig, patch_sig, success, meta FROM fix_records WHERE err_sig = ?1 ORDER BY timestamp DESC"
        )?;
        let records = stmt
            .query_map(params![err_sig], |row| {
                Ok(FixRecord {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    err_sig: row.get(2)?,
                    patch_sig: row.get(3)?,
                    success: row.get(4)?,
                    meta: row.get(5).ok(),
                })
            })?
            .filter_map(Result::ok)
            .collect();
        Ok(records)
    }
}

// Helper for current timestamp
pub fn current_timestamp() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

// API
pub fn insert(record: &FixRecord) -> Result<()> {
    let kb = KnowledgeBase::new("knowledge_base.db")?;
    kb.insert(record)
}

pub fn query(err_sig: &str) -> Result<Vec<FixRecord>> {
    let kb = KnowledgeBase::new("knowledge_base.db")?;
    kb.query(err_sig)
}
