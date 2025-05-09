//! 熔斷器 FuseBreaker
use std::fs::{File, OpenOptions};
use std::io::Write;
use chrono::Local;
use std::path::Path;
use std::process::Command;

pub struct FuseBreaker {
    fail_count: u32,
    threshold: u32,
    paused: bool,
    flag_path: String,
}

impl FuseBreaker {
    pub fn new(threshold: u32, flag_path: &str) -> Self {
        Self {
            fail_count: 0,
            threshold,
            paused: false,
            flag_path: flag_path.to_string(),
        }
    }
    /// 記錄一次失敗，若達標則熔斷
    pub fn record_failure(&mut self) {
        self.fail_count += 1;
        if self.fail_count >= self.threshold && !self.paused {
            self.trigger_fuse();
        }
    }
    /// 熔斷副作用：建立 pause.flag、git tag
    fn trigger_fuse(&mut self) {
        self.paused = true;
        // 建立 pause.flag
        let _ = File::create(&self.flag_path);
        // 建立 git tag
        let ts = Local::now().format("%Y%m%d-%H%M%S");
        let tag = format!("eva-fail-{}", ts);
        let _ = Command::new("git").args(["tag", &tag]).output();
    }
    /// 是否已熔斷
    pub fn is_paused(&self) -> bool {
        self.paused || Path::new(&self.flag_path).exists()
    }
    /// 取得目前失敗次數
    pub fn fail_count(&self) -> u32 {
        self.fail_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_fuse_breaker_triggers_on_3_failures() {
        let dir = tempdir().unwrap();
        let flag_path = dir.path().join("pause.flag");
        let mut fuse = FuseBreaker::new(3, flag_path.to_str().unwrap());
        assert!(!fuse.is_paused());
        fuse.record_failure();
        assert!(!fuse.is_paused());
        fuse.record_failure();
        assert!(!fuse.is_paused());
        fuse.record_failure();
        // 觸發熔斷
        assert!(fuse.is_paused());
        assert!(flag_path.exists());
        // 檢查 git tag 是否被執行（僅確認不 panic）
    }
}
