use std::path::Path;
use eva_system_core::{
    init_self_inspect,
    get_project_summary,
    get_recent_actions,
    generate_project_report,
    SelfInspectDB,
};

/// 系統自省菜單項目
enum SelfInspectMenuItem {
    Summary,
    DetailedReport,
    RecentChanges,
    Back,
}

/// 顯示系統自省菜單並處理選擇
pub fn run() {
    println!("\n====== EVA 系統自省 ======");
    
    // 初始化系統自省
    let proj_dir = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    let db_path = proj_dir.join(".eva/system.db").to_string_lossy().to_string();
    
    match init_self_inspect(&proj_dir.to_string_lossy(), &db_path) {
        Ok(db) => {
            loop {
                println!("\n請選擇操作：");
                println!("1) 系統狀態概要");
                println!("2) 詳細系統報告");
                println!("3) 最近系統異動記錄");
                println!("4) 返回上一頁");
                
                print!("請輸入選項：");
                std::io::Write::flush(&mut std::io::stdout()).ok();
                
                let mut choice = String::new();
                if std::io::stdin().read_line(&mut choice).is_err() {
                    println!("讀取輸入失敗，返回主選單");
                    break;
                }
                
                match choice.trim() {
                    "1" => show_summary(&db),
                    "2" => show_detailed_report(&db),
                    "3" => show_recent_changes(&db),
                    "4" => break,
                    _ => println!("無效選項，請重試"),
                }
            }
        },
        Err(e) => {
            println!("無法初始化系統自省：{:?}", e);
            println!("按 Enter 返回主選單");
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
        }
    }
}

/// 顯示系統狀態概要
fn show_summary(db: &SelfInspectDB) {
    println!("\n=== 系統狀態概要 ===");
    
    match get_project_summary(db) {
        Ok(summary) => {
            println!("{}", summary);
        },
        Err(e) => {
            println!("獲取系統狀態失敗：{:?}", e);
        }
    }
    
    println!("\n按 Enter 返回");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

/// 顯示詳細系統報告
fn show_detailed_report(db: &SelfInspectDB) {
    println!("\n=== 詳細系統報告 ===");
    
    match generate_project_report(db) {
        Ok(report) => {
            println!("{}", report);
        },
        Err(e) => {
            println!("生成系統報告失敗：{:?}", e);
        }
    }
    
    println!("\n按 Enter 返回");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

/// 顯示最近系統異動記錄
fn show_recent_changes(db: &SelfInspectDB) {
    println!("\n=== 最近系統異動記錄 ===");
    
    match get_recent_actions(db, 10) {
        Ok(actions) => {
            if actions.is_empty() {
                println!("目前沒有記錄到系統異動");
            } else {
                for action in actions {
                    let time = chrono::DateTime::<chrono::Utc>::from_timestamp(action.timestamp, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                        .unwrap_or_else(|| "未知時間".to_string());
                    
                    println!("[{}] {}: {}", 
                        time, action.action_type, action.description);
                }
            }
        },
        Err(e) => {
            println!("獲取系統異動記錄失敗：{:?}", e);
        }
    }
    
    println!("\n按 Enter 返回");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}
