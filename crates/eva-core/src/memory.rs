//! MemoryService middleware: ContextMemory (短期), VectorMemory (長期)
use std::fs::{OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::{Local, Utc};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

static MEM_LOG: Lazy<Mutex<std::fs::File>> = Lazy::new(|| {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open("memory.log")
        .expect("open memory.log")
        .into()
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub trace_id: String,
    pub role: String,
    pub text: String,
    pub ts: chrono::DateTime<Utc>,
}

/// 直接以 JSONL 方式追加一筆短期記憶
pub fn append(record: MemoryRecord) {
    let mut file = MEM_LOG.lock().unwrap();
    let line = serde_json::to_string(&record).expect("json");
    writeln!(file, "{line}").ok();

    // 向量化寫入資料庫（背景執行，失敗不影響主流程）
    let rec_clone = record.clone();
    tokio::spawn(async move {
        if let Err(e) = crate::memory_vector::insert(rec_clone).await {
            tracing::warn!("vector DB insert failed: {e}");
        }
    });
}

/// 短期記憶（僅儲存最近 N 筆）
pub struct ContextMemory {
    pub buffer: Vec<String>,
    pub capacity: usize,
}

impl ContextMemory {
    pub fn new(capacity: usize) -> Self {
        Self { buffer: Vec::with_capacity(capacity), capacity }
    }
    pub fn push(&mut self, record: String) {
        if self.buffer.len() >= self.capacity {
            self.buffer.remove(0);
        }
        self.buffer.push(record);
    }
    pub fn all(&self) -> &[String] {
        &self.buffer
    }
}

/// 長期記憶（寫入檔案，可擴充為向量 DB）
pub struct VectorMemory {
    pub log_path: PathBuf,
}

impl VectorMemory {
    pub fn new(log_path: PathBuf) -> Self {
        Self { log_path }
    }
    pub fn append(&self, record: &str) {
        let mut file = OpenOptions::new().create(true).append(true).open(&self.log_path).expect("open memory.log");
        writeln!(file, "{}", record).expect("write memory.log");
    }
}

/// MemoryService middleware
pub struct MemoryService {
    pub context: ContextMemory,
    pub vector: VectorMemory,
}

impl MemoryService {
    pub fn new(log_path: PathBuf, context_cap: usize) -> Self {
        Self {
            context: ContextMemory::new(context_cap),
            vector: VectorMemory::new(log_path),
        }
    }
    /// 執行 Plan 時記錄
    pub fn record(&mut self, plan: &str, intent: &str) {
        let ts = Local::now().to_rfc3339();
        let record = format!("{{\"ts\":\"{}\",\"plan\":\"{}\",\"intent\":\"{}\"}}", ts, plan, intent.replace('"', "'"));
        self.context.push(record.clone());
        self.vector.append(&record);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_memory_service_io() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("memory.log");
        let mut mem = MemoryService::new(log_path.clone(), 2);
        mem.record("RandomStart", "random start");
        mem.record("RandomStart", "hello");
        mem.record("RandomStart", "world");
        // 檢查 context buffer 僅有最近兩筆
        assert_eq!(mem.context.all().len(), 2);
        assert!(mem.context.all()[0].contains("hello"));
        assert!(mem.context.all()[1].contains("world"));
        // 檢查 log 檔案格式
        let content = fs::read_to_string(&log_path).unwrap();
        let lines: Vec<_> = content.lines().collect();
        assert_eq!(lines.len(), 3);
        for line in lines {
            assert!(line.contains("\"plan\":"));
            assert!(line.contains("\"intent\":"));
        }
    }
}
