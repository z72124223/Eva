//! Skeleton for IntentListener

use std::io::{self, Write};

pub struct IntentListener;

impl IntentListener {
    pub fn new() -> Self {
        IntentListener
    }
    /// 從 stdin 讀取一行，回傳意圖字串
    pub fn listen_stdin(&self) -> Option<String> {
        print!("請輸入指令: ");
        let _ = io::stdout().flush();
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(n) if n > 0 => Some(buf.trim().to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = IntentListener::new();
    }
    // listen_stdin 無法自動測試 stdin，略
}
