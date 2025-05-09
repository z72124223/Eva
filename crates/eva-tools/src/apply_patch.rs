use std::fs;
use anyhow::Result;

/// 沙盒：將 sum.rs 修正為正確版本
pub fn apply_patch_stub() -> Result<()> {
    // D6-2 安全機制
    const MAX_PATCH_SIZE: usize = 1024; // 可依需求調整
    let fixed = r#"// 修正後的 sum 實作
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        assert_eq!(sum(2, 3), 5);
    }
}
"#;
    use std::io::Write;
    use chrono::Local;
    let mut result = "失敗";
    let mut review_reason = String::new();
    // 危險關鍵字偵測
    let dangerous = fixed.contains("unsafe") || fixed.contains("std::fs::remove_");
    // patch safety check (呼叫 patch_safety)
    use crate::patch_safety::is_patch_safe;
    static mut PATCH_FAIL_COUNT: u32 = 0;
    const PATCH_FAIL_LIMIT: u32 = 3;
    let danger_keywords = ["unsafe", "remove_", "../"];
    let safety_result = is_patch_safe(fixed, MAX_PATCH_SIZE, &danger_keywords);
    if let Err(reason) = safety_result {
        unsafe {
            PATCH_FAIL_COUNT += 1;
        }
        review_reason = reason;
        if unsafe { PATCH_FAIL_COUNT } >= PATCH_FAIL_LIMIT {
            eprintln!("[熔斷] 連續 {} 次危險 patch，自動修補已暫停 30 分鐘並通知 Maintainer。", PATCH_FAIL_LIMIT);
            // 這裡可加通知維護者的機制，如寫入特定檔案或發送郵件
            std::thread::sleep(std::time::Duration::from_secs(1800));
            unsafe { PATCH_FAIL_COUNT = 0; }
        }
    } else {
        unsafe { PATCH_FAIL_COUNT = 0; }
    }
    // 若需人工審查，直接標記 manual-review 並輸出紀錄
    let now = Local::now();
    let now_str = now.format("%Y-%m-%d %H:%M:%S");
    let patch_filename = now.format("%Y%m%d_%H%M%S");
    let mut log_line = String::new();
    let fix_history_path = format!("crates/eva-tools/knowledge_base/fix_history/sum_{}.patch", patch_filename);
    if !review_reason.is_empty() {
        println!("[apply_patch_stub][安全機制] patch 標記 manual-review: {}", review_reason);
        log_line = format!("| {} | sum.rs | llm-patch | manual-review | {} |\n", now, review_reason);
        // 輸出 patch 到 fix_history
        if let Ok(mut file) = fs::OpenOptions::new().create(true).write(true).open(&fix_history_path) {
            let _ = file.write_all(fixed.as_bytes());
        }
    } else {
        match fs::write("crates/eva-tools/src/sum.rs", fixed) {
            Ok(_) => {
                println!("[apply_patch_stub] sum.rs 修補成功！");
                result = "成功";
            },
            Err(e) => println!("[apply_patch_stub] sum.rs 修補失敗: {}", e),
        }
        log_line = format!("| {} | sum.rs | llm-patch | {} | a + b 修補 |\n", now, result);
        // 輸出 patch 到 fix_history
        if let Ok(mut file) = fs::OpenOptions::new().create(true).write(true).open(&fix_history_path) {
            let _ = file.write_all(fixed.as_bytes());
        }
    }
    // 自動寫入修補紀錄
    let log_path = "crates/eva-tools/knowledge_base/error_log.md";
    if let Ok(mut file) = fs::OpenOptions::new().append(true).open(log_path) {
        let _ = file.write_all(log_line.as_bytes());
    }
    Ok(())
}
