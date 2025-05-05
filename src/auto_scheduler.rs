//! EVA 自動巡檢與自我修復排程器
//! 需在 main.rs 或 API Server 啟動時呼叫 auto_scheduler::start()
use tokio_cron_scheduler::{JobScheduler, Job};
use std::sync::Arc;
use std::path::PathBuf;

pub async fn start() {
    let sched = JobScheduler::new().await.unwrap();
    // 每 4h 檢查結構並修補新缺檔
    let j1 = Job::new_async("0 0 */4 * * *", |_uuid, _l| Box::pin(async {
        println!("[巡檢] check_structure --fix");
        crate::tools::check_structure::fix_missing();
    })).unwrap();
    sched.add(j1).await.unwrap();
    // 每 4h 為新檔產生簽名+測試
    let j2 = Job::new_async("0 10 */4 * * *", |_uuid, _l| Box::pin(async {
        println!("[巡檢] module_generator fill_stub + synthesize_tests");
        let missing = crate::tools::check_structure::find_todo_or_empty();
        crate::services::module_generator::fill_stub(&missing);
        crate::services::module_generator::synthesize_tests(&missing);
    })).unwrap();
    sched.add(j2).await.unwrap();
    // 每晚清理死依賴
    let j3 = Job::new_async("0 0 3 * * *", |_uuid, _l| Box::pin(async {
        println!("[巡檢] cargo udeps");
        let _ = std::process::Command::new("cargo").arg("udeps").status();
    })).unwrap();
    sched.add(j3).await.unwrap();
    // 每 30min 批量評估劇情品質
    let j4 = Job::new_async("0 */30 * * * *", |_uuid, _l| Box::pin(async {
        println!("[巡檢] boredom_detector batch");
        let scenes = crate::tools::check_structure::recent_scenes(10);
        crate::boredom_detector::batch_scene_quality(&scenes, 0.4);
    })).unwrap();
    sched.add(j4).await.unwrap();
    sched.start().await.unwrap();
}
