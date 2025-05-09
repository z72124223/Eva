//! 自省系統資料庫核心
//! 
//! 提供 SQLite 連接與 CRUD 操作

use crate::schema::{Module, FileEntry, ActionLog, now_ts};
use rusqlite::{params, Connection};
use std::path::Path;

/// 資料庫操作結果類型
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// 自省資料庫，封裝 SQLite 連線與操作
pub struct SelfInspectDB {
    pub conn: Connection,
}

impl SelfInspectDB {
    /// 建立新的自省資料庫連線
    pub fn new(db_path: &str) -> Result<Self> {
        // 確保資料庫目錄存在
        if let Some(parent) = Path::new(db_path).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }
    
    /// 初始化資料庫 schema
    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS modules (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                parent_id INTEGER,
                path TEXT NOT NULL UNIQUE,
                type TEXT NOT NULL,
                created_at INTEGER,
                updated_at INTEGER,
                FOREIGN KEY (parent_id) REFERENCES modules(id)
            );
            
            CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                module_id INTEGER,
                filename TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                filetype TEXT NOT NULL,
                size INTEGER,
                created_at INTEGER,
                updated_at INTEGER,
                deleted_at INTEGER,
                FOREIGN KEY (module_id) REFERENCES modules(id)
            );
            
            CREATE TABLE IF NOT EXISTS actions (
                id INTEGER PRIMARY KEY,
                action_type TEXT NOT NULL,
                target_type TEXT NOT NULL,
                target_id INTEGER,
                description TEXT,
                timestamp INTEGER
            );
            
            CREATE TABLE IF NOT EXISTS meta (
                key TEXT PRIMARY KEY,
                value TEXT
            );
            
            CREATE INDEX IF NOT EXISTS idx_modules_parent_id ON modules(parent_id);
            CREATE INDEX IF NOT EXISTS idx_files_module_id ON files(module_id);
            CREATE INDEX IF NOT EXISTS idx_files_filetype ON files(filetype);
            CREATE INDEX IF NOT EXISTS idx_actions_timestamp ON actions(timestamp);
            "#
        )?;
        Ok(())
    }
    
    // === 模組操作 ===
    
    /// 新增模組
    pub fn add_module(&self, module: &Module) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO modules (name, parent_id, path, type, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )?;
        
        stmt.execute(params![
            module.name,
            module.parent_id,
            module.path,
            module.mtype,
            module.created_at,
            module.updated_at
        ])?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    /// 更新模組
    pub fn update_module(&self, module: &Module) -> Result<()> {
        if module.id.is_none() {
            return Err("無法更新沒有 ID 的模組".into());
        }
        
        let now = now_ts();
        self.conn.execute(
            "UPDATE modules SET name = ?, parent_id = ?, path = ?, type = ?, updated_at = ?
             WHERE id = ?",
            params![
                module.name,
                module.parent_id,
                module.path,
                module.mtype,
                now,
                module.id
            ]
        )?;
        
        Ok(())
    }
    
    /// 刪除模組
    pub fn delete_module(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM modules WHERE id = ?", params![id])?;
        Ok(())
    }
    
    /// 尋找模組
    pub fn find_module_by_path(&self, path: &str) -> Result<Option<Module>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, parent_id, path, type, created_at, updated_at
             FROM modules WHERE path = ?"
        )?;
        
        let module = stmt.query_row(params![path], |row| {
            Ok(Module {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                parent_id: row.get(2)?,
                path: row.get(3)?,
                mtype: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        });
        
        match module {
            Ok(m) => Ok(Some(m)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    
    /// 計算模組數量
    pub fn count_modules(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM modules",
            [],
            |row| row.get(0)
        )?;
        
        Ok(count)
    }
    
    /// 計算特定類型的模組數量
    pub fn count_modules_by_type(&self, mtype: &str) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM modules WHERE type = ?",
            params![mtype],
            |row| row.get(0)
        )?;
        
        Ok(count)
    }
    
    // === 檔案操作 ===
    
    /// 新增檔案
    pub fn add_file(&self, file: &FileEntry) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO files (module_id, filename, path, filetype, size, created_at, updated_at, deleted_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )?;
        
        stmt.execute(params![
            file.module_id,
            file.filename,
            file.path,
            file.filetype,
            file.size,
            file.created_at,
            file.updated_at,
            file.deleted_at
        ])?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    /// 更新檔案
    pub fn update_file(&self, file: &FileEntry) -> Result<()> {
        if file.id.is_none() {
            return Err("無法更新沒有 ID 的檔案".into());
        }
        
        let now = now_ts();
        self.conn.execute(
            "UPDATE files SET module_id = ?, filename = ?, path = ?, filetype = ?, 
             size = ?, updated_at = ?, deleted_at = ? WHERE id = ?",
            params![
                file.module_id,
                file.filename,
                file.path,
                file.filetype,
                file.size,
                now,
                file.deleted_at,
                file.id
            ]
        )?;
        
        Ok(())
    }
    
    /// 標記檔案為已刪除
    pub fn mark_file_deleted(&self, id: i64) -> Result<()> {
        let now = now_ts();
        self.conn.execute(
            "UPDATE files SET deleted_at = ? WHERE id = ?",
            params![now, id]
        )?;
        
        Ok(())
    }
    
    /// 尋找檔案
    pub fn find_file_by_path(&self, path: &str) -> Result<Option<FileEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, module_id, filename, path, filetype, size, created_at, updated_at, deleted_at
             FROM files WHERE path = ?"
        )?;
        
        let file = stmt.query_row(params![path], |row| {
            Ok(FileEntry {
                id: Some(row.get(0)?),
                module_id: row.get(1)?,
                filename: row.get(2)?,
                path: row.get(3)?,
                filetype: row.get(4)?,
                size: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                deleted_at: row.get(8)?,
            })
        });
        
        match file {
            Ok(f) => Ok(Some(f)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    
    /// 計算檔案數量
    pub fn count_files(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM files WHERE deleted_at IS NULL",
            [],
            |row| row.get(0)
        )?;
        
        Ok(count)
    }
    
    /// 計算特定類型的檔案數量
    pub fn count_files_by_type(&self, filetype: &str) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM files WHERE filetype = ? AND deleted_at IS NULL",
            params![filetype],
            |row| row.get(0)
        )?;
        
        Ok(count)
    }
    
    // === 動作紀錄操作 ===
    
    /// 新增動作紀錄
    pub fn add_action(&self, action: &ActionLog) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO actions (action_type, target_type, target_id, description, timestamp)
             VALUES (?, ?, ?, ?, ?)"
        )?;
        
        stmt.execute(params![
            action.action_type,
            action.target_type,
            action.target_id,
            action.description,
            action.timestamp
        ])?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    /// 計算動作紀錄數量
    pub fn count_actions(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM actions",
            [],
            |row| row.get(0)
        )?;
        
        Ok(count)
    }
    
    // === 元資料操作 ===
    
    /// 設定元資料
    pub fn set_meta(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES (?, ?)",
            params![key, value]
        )?;
        
        Ok(())
    }
    
    /// 取得元資料
    pub fn get_meta(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM meta WHERE key = ?")?;
        
        let value = stmt.query_row(params![key], |row| {
            Ok(row.get(0)?)
        });
        
        match value {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
