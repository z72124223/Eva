//! TUI Dashboard for patch review (P3)
//! 提供 ncurses/ratatui 介面，讓用戶檢視、接受、拒絕 patch。

use std::fs;
use std::io::{self, Write};

/// Patch 狀態
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatchAction {
    Accept,
    Reject,
    Pending,
}

/// Patch 資訊結構
#[derive(Debug, Clone)]
pub struct PatchInfo {
    pub filename: String,
    pub content: String,
    pub status: PatchAction,
}

/// 讀取 fix_history 目錄的所有 patch
pub fn load_patches(dir: &str) -> io::Result<Vec<PatchInfo>> {
    let mut patches = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("patch") {
            let content = fs::read_to_string(&path)?;
            patches.push(PatchInfo {
                filename: path.file_name().unwrap().to_string_lossy().to_string(),
                content,
                status: PatchAction::Pending,
            });
        }
    }
    Ok(patches)
}

/// CLI/TUI 主流程（stub：僅 CLI 互動，未串接 ratatui）
pub fn dashboard_cli() -> io::Result<()> {
    let mut patches = load_patches("crates/eva-tools/knowledge_base/fix_history")?;
    if patches.is_empty() {
        println!("[dashboard] 尚無可審查的 patch。");
        return Ok(());
    }
    for (idx, patch) in patches.iter().enumerate() {
        println!("\n=== Patch #{}: {} ===", idx + 1, patch.filename);
        println!("{}", patch.content);
        print!("[a]接受 [r]拒絕 [Enter]略過: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "a" | "A" => {
                println!("[dashboard] 已接受 patch: {}", patch.filename);
                // TODO: 真正套用 patch
            },
            "r" | "R" => {
                println!("[dashboard] 已拒絕 patch: {}", patch.filename);
                // TODO: 標記為拒絕
            },
            _ => println!("[dashboard] 略過 patch: {}", patch.filename),
        }
    }
    Ok(())
}
