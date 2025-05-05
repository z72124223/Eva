use std::process::Command;
use crate::error_recovery;

/// 自動迴圈執行器主流程
/// 會自動執行 cargo check，遇錯誤自動修復，最多重試 MAX 次，失敗則標記需人工複查。
pub fn run_auto_loop_executor() -> std::io::Result<()> {
    let mut retries = 0;
    const MAX: usize = 5;
    loop {
        let status = Command::new("cargo").arg("check").output()?;
        if status.status.success() {
            println!("[auto-loop] 編譯通過，結束自動修復流程。");
            break;
        }
        println!("[auto-loop] 編譯失敗，自動呼叫 error_recovery::handle() 嘗試修復...");
        error_recovery::handle(&status.stderr)?;
        retries += 1;
        if retries > MAX {
            println!("[auto-loop] 已超過最大重試次數，標記需人工複查。");
            tag_for_review();
            break;
        }
    }
    Ok(())
}

/// 標記需人工複查（建立 Git branch）
fn tag_for_review() {
    use std::process::Command;
    use chrono::Local;
    let branch = format!("eva-auto-review-{}", Local::now().format("%Y%m%d%H%M%S"));
    let _ = Command::new("git").args(["checkout", "-b", &branch]).output();
    println!("[auto-loop] 已建立分支 {} 供人工複查。", branch);
}
