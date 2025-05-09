use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
// Enable chrono with serde features in Cargo.toml (already on) to serialize DateTime<Utc>.
/// 記憶資料，含 owner_id（唯一用戶）、scope（AI/人類可見性）、標籤等
pub struct Memory {
    pub id: i64,
    pub owner_id: String, // 唯一用戶識別
    pub user_id: String,  // 向後相容，未來可移除
    pub memory_type: String, // e.g. "chat", "preference", "task", "log"
    pub content: String,     // 可用 JSON 或純文字
    pub created_at: DateTime<Utc>,
    pub scope: String,       // "ai-visible" | "human-only" | "private" 等
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryQuery {
    pub user_id: Option<String>,
    pub memory_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
}

#[derive(Debug)]
pub enum MemoryError {
    Database(String),
    NotFound,
    Serialize(String),
    Permission(String),
}

pub trait MemoryStore {
    fn add_memory(&self, mem: &Memory) -> Result<i64, MemoryError>;
    fn get_memories(&self, query: MemoryQuery) -> Result<Vec<Memory>, MemoryError>;
    fn update_memory(&self, id: i64, new_content: &str, tags: Option<Vec<String>>) -> Result<(), MemoryError>;
    fn delete_memory(&self, id: i64) -> Result<(), MemoryError>;
}
