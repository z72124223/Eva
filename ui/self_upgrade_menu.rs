//! CLI 介面，讓人或 AI 執行 check/generate 等自升級動作。
use std::io::{self, Write};

pub enum SelfUpgradeAction {
    Check,
    Generate,
    AiFillSkeleton,
    AiFullAuto, // 完全自動化：AI 完善內容（fill_stub + synthesize_tests）
    Exit,
}

pub fn menu() -> SelfUpgradeAction {
    println!("=== EVA Self-Upgrade Menu ===");
    println!("1) 檢查結構 (check)");
    println!("2) 自動補檔 (generate)");
    println!("3) AI 填充骨架 (AI fill skeleton)");
    println!("4) AI 完善函式內容（完全自動化 fill_stub + synthesize_tests）");
    println!("q) 離開 (exit)");
    print!("請輸入選項: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    match buf.trim() {
        "1" => SelfUpgradeAction::Check,
        "2" => SelfUpgradeAction::Generate,
        "3" => SelfUpgradeAction::AiFillSkeleton,
        "4" => SelfUpgradeAction::AiFullAuto,
        _ => SelfUpgradeAction::Exit,
    }
}
