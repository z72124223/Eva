//! EVA 系統核心 - 自省與模組管理系統
//! 
//! 本 crate 提供 EVA 系統自省能力，讓 EVA 能感知自身結構、模組異動
//! 並支援查詢、追蹤、自我修復等進階功能

mod schema;
mod scanner;
mod db;
mod query;

pub use schema::{Module, FileEntry, ActionLog, Meta};
pub use db::SelfInspectDB;
pub use query::*;
pub use scanner::*;

/// 初始化自省系統，傳入專案根目錄和 DB 路徑
pub fn init_self_inspect(project_root: &str, db_path: &str) -> db::Result<SelfInspectDB> {
    let db = SelfInspectDB::new(db_path)?;
    db.init_schema()?;
    
    // 當沒有模組資料時，掃描並建立初始資料
    if db.count_modules()? == 0 {
        let scanner = scanner::ProjectScanner::new(project_root);
        scanner.scan(&db)?;
    }
    
    Ok(db)
}

/// 快速取得專案清單，建議從主選單調用
pub fn get_project_summary(db: &SelfInspectDB) -> db::Result<String> {
    let crates = db.count_modules_by_type("crate")?;
    let modules = db.count_modules()?;
    let files = db.count_files()?;
    let code_files = db.count_files_by_type("rs")?;
    let actions = db.count_actions()?;
    
    Ok(format!(
        "EVA 專案結構：{} crates、{} 模組、{} 檔案（其中 {} 為程式碼）\n最近異動：{} 筆",
        crates, modules, files, code_files, actions
    ))
}

/// 檢查是否有新增或刪除的模組（自我監控）
pub fn check_for_changes(db: &SelfInspectDB, project_root: &str) -> db::Result<bool> {
    let scanner = scanner::ProjectScanner::new(project_root);
    scanner.detect_changes(db)
}
