//! 自省系統查詢模組
//! 
//! 提供高階查詢、報表與統計功能

use crate::db::{Result, SelfInspectDB};
use crate::schema::{Module, FileEntry, ActionLog};
use rusqlite::params;

/// 模組樹節點，用於構建階層式模組結構
#[derive(Debug, Clone)]
pub struct ModuleNode {
    /// 模組資訊
    pub module: Module,
    /// 子模組
    pub children: Vec<ModuleNode>,
    /// 檔案
    pub files: Vec<FileEntry>,
}

/// 專案統計資訊
#[derive(Debug, Clone)]
pub struct ProjectStats {
    /// 模組總數
    pub module_count: i64,
    /// 檔案總數
    pub file_count: i64,
    /// Rust 檔案數量
    pub rust_file_count: i64,
    /// 總程式碼大小 (位元組)
    pub total_code_size: i64,
    /// 最近異動數量
    pub recent_action_count: i64,
}

/// 查詢模組樹結構
pub fn get_module_tree(db: &SelfInspectDB) -> Result<Vec<ModuleNode>> {
    // 先取得所有頂層模組 (parent_id 為 NULL)
    let top_modules = get_modules_by_parent(db, None)?;
    
    // 建立模組樹
    let mut result = Vec::new();
    for module in top_modules {
        let node = build_module_tree(db, module)?;
        result.push(node);
    }
    
    Ok(result)
}

/// 遞迴構建模組樹
fn build_module_tree(db: &SelfInspectDB, module: Module) -> Result<ModuleNode> {
    let module_id = module.id.ok_or("模組缺少 ID")?;
    
    // 取得子模組
    let children_modules = get_modules_by_parent(db, Some(module_id))?;
    
    // 遞迴處理子模組
    let mut children = Vec::new();
    for child_module in children_modules {
        let child_node = build_module_tree(db, child_module)?;
        children.push(child_node);
    }
    
    // 取得模組的檔案
    let files = get_files_by_module(db, module_id)?;
    
    Ok(ModuleNode {
        module,
        children,
        files,
    })
}

/// 取得某父模組的所有子模組
fn get_modules_by_parent(db: &SelfInspectDB, parent_id: Option<i64>) -> Result<Vec<Module>> {
    let conn = &db.conn;
    let sql = match parent_id {
        Some(_) => "SELECT id, name, parent_id, path, type, created_at, updated_at FROM modules WHERE parent_id = ?",
        None => "SELECT id, name, parent_id, path, type, created_at, updated_at FROM modules WHERE parent_id IS NULL",
    };
    
    let mut stmt = conn.prepare(sql)?;
    
    let modules: Vec<Module> = if let Some(id) = parent_id {
        let iter = stmt.query_map(params![id], |row| {
            Ok(Module {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                parent_id: row.get(2)?,
                path: row.get(3)?,
                mtype: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        iter.collect::<std::result::Result<Vec<_>, _>>()?
    } else {
        let iter = stmt.query_map([], |row| {
            Ok(Module {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                parent_id: row.get(2)?,
                path: row.get(3)?,
                mtype: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        iter.collect::<std::result::Result<Vec<_>, _>>()?
    };
    
    Ok(modules)
}

/// 取得某模組的所有檔案
fn get_files_by_module(db: &SelfInspectDB, module_id: i64) -> Result<Vec<FileEntry>> {
    let conn = &db.conn;
    let mut stmt = conn.prepare(
        "SELECT id, module_id, filename, path, filetype, size, created_at, updated_at, deleted_at
         FROM files WHERE module_id = ? AND deleted_at IS NULL"
    )?;
    
    let file_iter = stmt.query_map(params![module_id], |row| {
        Ok(FileEntry {
            id: Some(row.get(0)?),
            module_id: row.get(1)?,
            filename: row.get(2)?,
            path: row.get(3)?,
            filetype: row.get(4)?,
            size: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            deleted_at: row.get(8)?,
        })
    })?;
    
    let mut files = Vec::new();
    for file in file_iter {
        files.push(file?);
    }
    
    Ok(files)
}

/// 取得專案統計資訊
pub fn get_project_stats(db: &SelfInspectDB) -> Result<ProjectStats> {
    let module_count = db.count_modules()?;
    let file_count = db.count_files()?;
    let rust_file_count = db.count_files_by_type("rs")?;
    
    // 計算總程式碼大小
    let conn = &db.conn;
    let total_size: i64 = conn.query_row(
        "SELECT SUM(size) FROM files WHERE deleted_at IS NULL",
        [],
        |row| row.get(0)
    )?;
    
    // 取得最近一週的異動數量
    let week_ago = chrono::Utc::now().timestamp() - 7 * 24 * 60 * 60;
    let recent_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM actions WHERE timestamp > ?",
        params![week_ago],
        |row| row.get(0)
    )?;
    
    Ok(ProjectStats {
        module_count,
        file_count,
        rust_file_count,
        total_code_size: total_size,
        recent_action_count: recent_count,
    })
}

/// 搜尋檔案
pub fn search_files(db: &SelfInspectDB, query: &str) -> Result<Vec<FileEntry>> {
    let conn = &db.conn;
    let query_param = format!("%{}%", query);
    
    let mut stmt = conn.prepare(
        "SELECT id, module_id, filename, path, filetype, size, created_at, updated_at, deleted_at
         FROM files 
         WHERE (filename LIKE ? OR path LIKE ?) AND deleted_at IS NULL"
    )?;
    
    let file_iter = stmt.query_map(params![query_param, query_param], |row| {
        Ok(FileEntry {
            id: Some(row.get(0)?),
            module_id: row.get(1)?,
            filename: row.get(2)?,
            path: row.get(3)?,
            filetype: row.get(4)?,
            size: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            deleted_at: row.get(8)?,
        })
    })?;
    
    let mut files = Vec::new();
    for file in file_iter {
        files.push(file?);
    }
    
    Ok(files)
}

/// 取得最近異動記錄
pub fn get_recent_actions(db: &SelfInspectDB, limit: usize) -> Result<Vec<ActionLog>> {
    let conn = &db.conn;
    let mut stmt = conn.prepare(
        "SELECT id, action_type, target_type, target_id, description, timestamp
         FROM actions 
         ORDER BY timestamp DESC
         LIMIT ?"
    )?;
    
    let action_iter = stmt.query_map(params![limit as i64], |row| {
        Ok(ActionLog {
            id: Some(row.get(0)?),
            action_type: row.get(1)?,
            target_type: row.get(2)?,
            target_id: row.get(3)?,
            description: row.get(4)?,
            timestamp: row.get(5)?,
        })
    })?;
    
    let mut actions = Vec::new();
    for action in action_iter {
        actions.push(action?);
    }
    
    Ok(actions)
}

/// 產生完整的專案報告文字
pub fn generate_project_report(db: &SelfInspectDB) -> Result<String> {
    let stats = get_project_stats(db)?;
    let recent_actions = get_recent_actions(db, 5)?;
    
    let mut report = String::new();
    report.push_str("=== EVA 系統自省報告 ===\n\n");
    
    // 基本統計
    report.push_str(&format!("模組總數: {}\n", stats.module_count));
    report.push_str(&format!("檔案總數: {}\n", stats.file_count));
    report.push_str(&format!("Rust 檔案: {}\n", stats.rust_file_count));
    report.push_str(&format!("程式碼大小: {} KB\n", stats.total_code_size / 1024));
    report.push_str(&format!("最近一週異動: {}\n\n", stats.recent_action_count));
    
    // 最近異動
    report.push_str("最近異動:\n");
    for action in recent_actions {
        let time = chrono::DateTime::<chrono::Utc>::from_timestamp(action.timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_else(|| "未知時間".to_string());
        
        report.push_str(&format!("- [{}] {}: {}\n", 
            time, action.action_type, action.description));
    }
    
    Ok(report)
}
