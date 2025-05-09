//! 專案掃描器
//! 
//! 提供掃描專案結構、檢測變更的功能

use crate::db::{Result, SelfInspectDB};
use crate::schema::{Module, FileEntry, ActionLog};
use std::collections::HashSet;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 專案掃描器，負責掃描專案並更新 DB
pub struct ProjectScanner {
    /// 專案根目錄
    root: PathBuf,
    /// 要忽略的目錄
    ignored_dirs: HashSet<String>,
}

impl ProjectScanner {
    /// 建立新掃描器
    pub fn new(root_path: &str) -> Self {
        let mut ignored_dirs = HashSet::new();
        // 忽略常見的非源碼目錄
        ignored_dirs.insert(".git".to_string());
        ignored_dirs.insert("target".to_string());
        ignored_dirs.insert("node_modules".to_string());
        
        Self {
            root: PathBuf::from(root_path),
            ignored_dirs,
        }
    }
    
    /// 掃描專案結構並寫入資料庫
    pub fn scan(&self, db: &SelfInspectDB) -> Result<()> {
        // 掃描所有 src 目錄
        let src_path = self.root.join("src");
        if src_path.exists() {
            self.scan_directory(db, &src_path, None, "src")?;
        }
        
        // 掃描所有 crates 目錄
        let crates_path = self.root.join("crates");
        if crates_path.exists() {
            for entry in std::fs::read_dir(crates_path)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    // 為每個 crate 建立一個模組
                    let module = Module::new(&name, None, &path.to_string_lossy(), "crate");
                    let module_id = db.add_module(&module)?;
                    
                    // 記錄動作
                    let action = ActionLog::new(
                        "add", "module", Some(module_id), 
                        &format!("發現 crate: {}", name)
                    );
                    db.add_action(&action)?;
                    
                    // 遞迴掃描 crate 內容
                    let src_dir = path.join("src");
                    if src_dir.exists() {
                        self.scan_directory(db, &src_dir, Some(module_id), "module")?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 遞迴掃描目錄
    fn scan_directory(&self, db: &SelfInspectDB, dir: &Path, parent_id: Option<i64>, mtype: &str) -> Result<()> {
        for entry in WalkDir::new(dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| !self.is_ignored(e.path()))
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            if path.is_dir() {
                // 為目錄建立模組
                let module = Module::new(&name, parent_id, &path.to_string_lossy(), "directory");
                let module_id = db.add_module(&module)?;
                
                // 遞迴掃描子目錄
                self.scan_directory(db, path, Some(module_id), mtype)?;
            } else {
                // 為檔案建立紀錄
                let filetype = path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_string();
                
                let metadata = std::fs::metadata(path)?;
                
                let file = FileEntry::new(
                    parent_id, 
                    &name, 
                    &path.to_string_lossy(), 
                    &filetype,
                    metadata.len() as i64
                );
                
                db.add_file(&file)?;
            }
        }
        
        Ok(())
    }
    
    /// 檢測是否有檔案或目錄變更
    pub fn detect_changes(&self, db: &SelfInspectDB) -> Result<bool> {
        // 1. 檢查新增或修改的檔案
        let src_change = self.detect_file_changes(db, &self.root.join("src"))?;
        let crate_change = self.detect_file_changes(db, &self.root.join("crates"))?;
        
        // 2. 檢查刪除的檔案
        let del_change = self.detect_deleted_files(db)?;
        
        Ok(src_change || crate_change || del_change)
    }
    
    /// 檢測新增或修改的檔案
    fn detect_file_changes(&self, db: &SelfInspectDB, dir: &Path) -> Result<bool> {
        let mut changes_detected = false;
        
        if !dir.exists() {
            return Ok(false);
        }
        
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| !self.is_ignored(e.path()))
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.is_file() {
                let file_path = path.to_string_lossy().to_string();
                
                // 檢查檔案是否存在於資料庫
                if let Some(existing_file) = db.find_file_by_path(&file_path)? {
                    // 檢查檔案是否已被標記為刪除
                    if existing_file.deleted_at.is_some() {
                        // 檔案之前被標記為刪除，但現在又存在了
                        let mut updated_file = existing_file.clone();
                        updated_file.deleted_at = None;
                        db.update_file(&updated_file)?;
                        
                        // 記錄復原動作
                        let action = ActionLog::new(
                            "restore", "file", existing_file.id, 
                            &format!("檔案復原: {}", file_path)
                        );
                        db.add_action(&action)?;
                        changes_detected = true;
                    } else {
                        // 檢查檔案大小是否變更
                        let metadata = std::fs::metadata(path)?;
                        if metadata.len() as i64 != existing_file.size {
                            let mut updated_file = existing_file.clone();
                            updated_file.size = metadata.len() as i64;
                            db.update_file(&updated_file)?;
                            
                            // 記錄更新動作
                            let action = ActionLog::new(
                                "update", "file", existing_file.id, 
                                &format!("檔案更新: {}", file_path)
                            );
                            db.add_action(&action)?;
                            changes_detected = true;
                        }
                    }
                } else {
                    // 新檔案，加入資料庫
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    let filetype = path.extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let metadata = std::fs::metadata(path)?;
                    
                    // 找出父模組
                    let parent_dir = path.parent().unwrap_or(Path::new(""));
                    let parent_id = if let Some(module) = db.find_module_by_path(&parent_dir.to_string_lossy())? {
                        module.id
                    } else {
                        None
                    };
                    
                    let file = FileEntry::new(
                        parent_id, 
                        &name, 
                        &file_path, 
                        &filetype,
                        metadata.len() as i64
                    );
                    
                    let file_id = db.add_file(&file)?;
                    
                    // 記錄新增動作
                    let action = ActionLog::new(
                        "add", "file", Some(file_id), 
                        &format!("新增檔案: {}", file_path)
                    );
                    db.add_action(&action)?;
                    changes_detected = true;
                }
            }
        }
        
        Ok(changes_detected)
    }
    
    /// 檢測刪除的檔案
    fn detect_deleted_files(&self, _db: &SelfInspectDB) -> Result<bool> {
        // 這裡需要實現檢測哪些檔案已經從檔案系統中刪除
        // 由於這個實作相當複雜，這裡僅提供框架
        // 完整實作需要從資料庫獲取所有檔案列表，然後檢查每一個是否仍存在
        
        // 此處為簡化版，僅返回 false 表示沒有變更
        // 真實實作應該檢查資料庫中所有標記為非刪除的檔案是否實際存在
        
        Ok(false)
    }
    
    /// 檢查路徑是否應該被忽略
    fn is_ignored(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| self.ignored_dirs.contains(name))
            .unwrap_or(false)
            || path.to_string_lossy().contains("target")
    }
}
