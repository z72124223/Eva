use std::process::Command;
use crate::error_recovery;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use tokio_cron_scheduler::{JobScheduler, Job};
use std::thread;
use eva_tools::fine_tune;

/// Daemon + watcher + cron executor (async)
pub async fn run_auto_loop_daemon() {
    // 啟動 watcher 任務
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2)).expect("建立 watcher 失敗");
    watcher.watch("src", RecursiveMode::Recursive).expect("監聽 src/ 失敗");
    watcher.watch("tests", RecursiveMode::Recursive).expect("監聽 tests/ 失敗");
    println!("[daemon] 監聽 src/ 與 tests/ 目錄異動...");

    // 啟動 cron 任務（每日凌晨 3:15 執行 fine-tune stub）
    let sched = JobScheduler::new().await.expect("建立 scheduler 失敗");

    let job = Job::new_async("15 3 * * *", |_uuid, _l| {
        Box::pin(async move {
            println!("[cron] 定時觸發 fine-tune stub (每日 03:15)");
            match fine_tune::run_nightly_fine_tune() {
                Ok(_) => println!("[cron] Fine-tuning completed successfully."),
                Err(e) => println!("[cron][error] Fine-tuning failed: {}", e),
            }
        })
    }).expect("建立 cron job 失敗");

    sched.add(job).await.expect("新增 cron 任務失敗");
    sched.start().await.expect("啟動 scheduler 失敗");

    // watcher 事件處理（blocking）
    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(path)) | Ok(DebouncedEvent::Create(path)) | Ok(DebouncedEvent::Remove(path)) => {
                println!("[watcher] 偵測到 {:?} 異動，觸發自動修補流程...", path);
                // 同步呼叫原 auto-loop 修補流程
                let _ = run_auto_loop_executor();
            },
            Ok(_) => {},
            Err(e) => println!("[watcher] 監聽錯誤: {:?}", e),
        }
    }
}



/// 原同步修補流程（保留重試與標記分支）
pub fn run_auto_loop_executor() -> std::io::Result<()> {
    let mut retries = 0;
    const MAX: usize = 5;
    loop {
        let output = Command::new("cargo").arg("test").output()?;
        if output.status.success() {
            println!("[auto-loop] 測試通過，結束自動修復流程。");
            break;
        }
        println!("[auto-loop] 測試失敗，自動呼叫 error_recovery::handle() 嘗試修復...");
        error_recovery::handle(&output.stdout, &output.stderr)?;
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
    use chrono::Local;
    let branch = format!("eva-auto-review-{}", Local::now().format("%Y%m%d%H%M%S"));
    let _ = Command::new("git").args(["checkout", "-b", &branch]).output();
    println!("[auto-loop] 已建立分支 {} 供人工複查。", branch);
}
