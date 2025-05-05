//! 根據 ExpectedPath 建立 .rs 檔，並自動更新父 mod.rs。
use std::fs;
use std::path::{Path, PathBuf};

/// 自動偵測 TODO/空函式，AI 產生內容填回檔案
pub fn fill_stub(targets: &[PathBuf]) {
    use std::fs::{read_to_string, write, copy};
    use crate::services::prompt_adapter::{PromptAdapter, LlmProvider};
    let prompt_adapter = PromptAdapter::new();
    for path in targets {
        if !path.exists() { continue; }
        if let Ok(content) = read_to_string(path) {
            let mut new_content = String::new();
            let mut changed = false;
            for line in content.lines() {
                if line.contains("TODO") || line.trim().ends_with("{}") {
                    let prompt = format!("請根據此 Rust 檔案內容與 TODO/空函式區段，自動補齊函式簽名與 doc：\n{}", content);
                    let llm = prompt_adapter.infer(&prompt, LlmProvider::OpenAi);
                    new_content.push_str(&llm);
                    new_content.push('\n');
                    changed = true;
                } else {
                    new_content.push_str(line);
                    new_content.push('\n');
                }
            }
            if changed {
                let backup = format!("{}.bak", path.display());
                copy(path, &backup).ok();
                write(path, &new_content).ok();
                // 格式化+檢查
                let fmt_ok = std::process::Command::new("cargo").arg("fmt").status().map(|s| s.success()).unwrap_or(false);
                let check_ok = std::process::Command::new("cargo").arg("check").status().map(|s| s.success()).unwrap_or(false);
                if !fmt_ok || !check_ok {
                    // 回滾
                    copy(&backup, path).ok();
                    println!("[回滾] {} 格式化或編譯失敗，已還原。", path.display());
                } else {
                    println!("[AI填充] {} 已自動補齊 TODO/空函式並通過檢查。", path.display());
                }
            }
        }
    }
}

/// 自動為 public API 產生最小單元測試；生成失敗則回滾
pub fn synthesize_tests(targets: &[PathBuf]) {
    use std::fs::{read_to_string, write, copy};
    use crate::services::prompt_adapter::{PromptAdapter, LlmProvider};
    let prompt_adapter = PromptAdapter::new();
    for path in targets {
        if !path.exists() { continue; }
        if let Ok(content) = read_to_string(path) {
            let prompt = format!("請根據此 Rust 檔案內容，自動為 public API 產生最小單元測試模組 #[cfg(test)]，並附上簡要註解：\n{}", content);
            let llm = prompt_adapter.infer(&prompt, LlmProvider::OpenAi);
            if llm.trim().is_empty() { continue; }
            let mut new_content = content.clone();
            new_content.push_str("\n");
            new_content.push_str(&llm);
            let backup = format!("{}.bak", path.display());
            copy(path, &backup).ok();
            write(path, &new_content).ok();
            // 格式化+檢查+測試
            let fmt_ok = std::process::Command::new("cargo").arg("fmt").status().map(|s| s.success()).unwrap_or(false);
            let check_ok = std::process::Command::new("cargo").arg("check").status().map(|s| s.success()).unwrap_or(false);
            let test_ok = std::process::Command::new("cargo").arg("nextest").arg("run").status().map(|s| s.success()).unwrap_or(false);
            if !fmt_ok || !check_ok || !test_ok {
                // 回滾
                copy(&backup, path).ok();
                println!("[回滾] {} 產生測試後驗證失敗，已還原。", path.display());
            } else {
                println!("[AI測試] {} 已自動產生測試並通過驗證。", path.display());
            }
        }
    }
}

/// 建立缺漏的 .rs 檔案
pub fn create_missing_rs(paths: &[PathBuf]) {
    use std::process::Command;
    use std::fs::{read_to_string, write, copy};
    use crate::services::prompt_adapter::{PromptAdapter, LlmProvider};
    let prompt_adapter = PromptAdapter::new();
    for path in paths {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let fname = path.file_stem().unwrap().to_string_lossy();
        // 若檔案已存在，檢查 TODO 區段
        if path.exists() {
            if let Ok(content) = read_to_string(path) {
                let mut new_content = String::new();
                let mut changed = false;
                for line in content.lines() {
                    if line.contains("TODO") {
                        let prompt = format!("請根據此 Rust 檔案內容與 TODO 區段，自動補齊函式簽名與 doc：\n{}", content);
                        let llm = prompt_adapter.infer(&prompt, LlmProvider::OpenAi);
                        new_content.push_str(&llm);
                        new_content.push('\n');
                        changed = true;
                    } else {
                        new_content.push_str(line);
                        new_content.push('\n');
                    }
                }
                if changed {
                    let backup = format!("{}.bak", path.display());
                    copy(path, &backup).ok();
                    write(path, &new_content).ok();
                    // 格式化+檢查
                    let fmt_ok = Command::new("cargo").arg("fmt").status().map(|s| s.success()).unwrap_or(false);
                    let check_ok = Command::new("cargo").arg("check").status().map(|s| s.success()).unwrap_or(false);
                    if !fmt_ok || !check_ok {
                        // 回滾
                        copy(&backup, path).ok();
                        println!("[回滾] {} 格式化或編譯失敗，已還原。", path.display());
                    } else {
                        println!("[AI填充] {} 已自動補齊 TODO 並通過檢查。", path.display());
                    }
                }
            }
        } else {
            // 新檔案，直接 LLM 產生骨架
            let prompt = format!("請根據檔名 {} 產生一份 Rust 檔案骨架（含最基本函式簽名與註解）", fname);
            let content = prompt_adapter.infer(&prompt, LlmProvider::OpenAi);
            let content = if content.trim().is_empty() { "// auto-generated\n".to_string() } else { content };
            write(path, &content).ok();
            // 同步更新父 mod.rs
            if let Some(parent) = path.parent() {
                let mod_path = parent.join("mod.rs");
                let mod_decl = format!("pub mod {};", fname);
                if mod_path.exists() {
                    let old = read_to_string(&mod_path).unwrap();
                    if !old.contains(&mod_decl) {
                        let mut new = old;
                        new.push_str("\n");
                        new.push_str(&mod_decl);
                        write(&mod_path, new).ok();
                    }
                } else {
                    write(&mod_path, format!("{}\n", mod_decl)).ok();
                }
            }
            // 格式化+檢查
            let fmt_ok = Command::new("cargo").arg("fmt").status().map(|s| s.success()).unwrap_or(false);
            let check_ok = Command::new("cargo").arg("check").status().map(|s| s.success()).unwrap_or(false);
            if !fmt_ok || !check_ok {
                println!("[警告] {} 補檔後格式化或編譯失敗，請檢查檔案內容。", path.display());
            } else {
                println!("[補檔] {} 已自動產生並通過檢查。", path.display());
            }
        }
    }
}


