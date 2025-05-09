//! 定義 EVA 自省系統資料庫架構
//! 
//! 封裝所有資料表結構與相關型別

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// 取得目前時間戳（Unix timestamp, 秒）
pub fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// 模組資訊，用於表示 crate、module 或子目錄
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    /// 資料庫 ID
    pub id: Option<i64>,
    /// 模組名稱
    pub name: String,
    /// 父模組 ID，若為頂層則為 None
    pub parent_id: Option<i64>,
    /// 檔案系統路徑
    pub path: String,
    /// 模組類型：crate, module, directory, ...
    pub mtype: String,
    /// 建立時間
    pub created_at: i64,
    /// 更新時間
    pub updated_at: i64,
}

impl Module {
    /// 建立新模組
    pub fn new(name: &str, parent_id: Option<i64>, path: &str, mtype: &str) -> Self {
        let now = now_ts();
        Self {
            id: None,
            name: name.to_string(),
            parent_id,
            path: path.to_string(),
            mtype: mtype.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// 檔案資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// 資料庫 ID
    pub id: Option<i64>,
    /// 所屬模組 ID
    pub module_id: Option<i64>,
    /// 檔案名稱
    pub filename: String,
    /// 檔案路徑
    pub path: String,
    /// 檔案類型 (副檔名)
    pub filetype: String,
    /// 檔案大小 (bytes)
    pub size: i64,
    /// 建立時間
    pub created_at: i64,
    /// 更新時間
    pub updated_at: i64,
    /// 刪除時間，若未刪除則為 None
    pub deleted_at: Option<i64>,
}

impl FileEntry {
    /// 建立新檔案
    pub fn new(module_id: Option<i64>, filename: &str, path: &str, filetype: &str, size: i64) -> Self {
        let now = now_ts();
        Self {
            id: None,
            module_id,
            filename: filename.to_string(),
            path: path.to_string(),
            filetype: filetype.to_string(),
            size,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}

/// 動作紀錄，追蹤系統異動
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionLog {
    /// 資料庫 ID
    pub id: Option<i64>,
    /// 動作類型: add, remove, update, move, ...
    pub action_type: String,
    /// 目標類型: module, file, ...
    pub target_type: String,
    /// 目標 ID
    pub target_id: Option<i64>,
    /// 說明文字
    pub description: String,
    /// 時間戳
    pub timestamp: i64,
}

impl ActionLog {
    /// 建立新動作紀錄
    pub fn new(action_type: &str, target_type: &str, target_id: Option<i64>, description: &str) -> Self {
        Self {
            id: None,
            action_type: action_type.to_string(),
            target_type: target_type.to_string(),
            target_id,
            description: description.to_string(),
            timestamp: now_ts(),
        }
    }
}

/// 元資料
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// 鍵
    pub key: String,
    /// 值
    pub value: String,
}

impl Meta {
    /// 建立新元資料
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}
