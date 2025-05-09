//! Feedback utils - 不同模組共用回饋資料格式
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackRecord {
    pub prompt: String,
    pub answer: String,
    pub good: bool,
    pub timestamp: String,
}
