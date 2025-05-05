use std::path::Path;
use std::env;

mod sum;
mod apply_patch;
use sum::sum;

mod check_structure;
mod module_generator;
mod auto_generate;
mod auto_loop_executor;
mod api_server;
pub mod error_recovery;
mod prompt_adapter;

use auto_generate::run_auto_generate;

/// 主程式進入點
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // 進入互動式主選單
        loop {
            println!("\n=== EVA Tools 主選單 ===");
            println!("[1] 檢查結構");
            println!("[2] 產生骨架");
            println!("[3] 啟動 HTTP API 伺服器");
            println!("[q] 離開");
            print!("請輸入選項: ");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "1" => run_check_structure(),
                "2" => run_auto_generate(),
                "3" => {
                    println!("啟動 EVA HTTP API 伺服器於 http://127.0.0.1:8080 ...");
                    // 必須為 async，這裡用 block_on
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(eva_api_server());
                    break;
                },
                "q" | "Q" => { println!("再見！"); break; },
                _ => println!("無效選項，請重新輸入。"),
            }
        }
        return;
    }
    let cmd = &args[1];
    if cmd == "create-fix-pr" || (cmd == "create-fix-pr" && args.get(2).map(|s| s == "--ci").unwrap_or(false)) {
        use std::process::{Command, Stdio};
        use chrono::Local;
        use std::fs;
        // 0. 檢查 git config
        let check_name = Command::new("git").args(["config", "user.name"]).output().unwrap();
        if !check_name.status.success() || String::from_utf8_lossy(&check_name.stdout).trim().is_empty() {
            println!("[create-fix-pr] 未設置 git user.name，嘗試自動設置...");
            let _ = Command::new("git").args(["config", "user.name", "eva-bot"]).status();
        }
        let check_email = Command::new("git").args(["config", "user.email"]).output().unwrap();
        if !check_email.status.success() || String::from_utf8_lossy(&check_email.stdout).trim().is_empty() {
            println!("[create-fix-pr] 未設置 git user.email，嘗試自動設置...");
            let _ = Command::new("git").args(["config", "user.email", "eva-bot@localhost"]).status();
        }
        // 1. 產生 patch
        println!("[create-fix-pr] 產生 patch 檔...");
        let patch_status = Command::new("git")
            .args(["diff"]).stdout(Stdio::piped()).output().expect("無法執行 git diff");
        fs::write("eva_auto_fix.patch", &patch_status.stdout).expect("寫入 patch 失敗");
        // 2. 建立分支並 commit
        let branch = format!("auto-review-{}", Local::now().format("%Y%m%d%H%M%S"));
        println!("[create-fix-pr] 建立分支 {} 並 commit 修補內容...", branch);
        let _ = Command::new("git").args(["checkout", "-b", &branch]).status();
        let _ = Command::new("git").args(["add", "."]).status();
        // 檢查是否有 staged 內容
        let diff_index = Command::new("git").args(["diff", "--cached", "--name-only"]).output().unwrap();
        if String::from_utf8_lossy(&diff_index.stdout).trim().is_empty() {
            eprintln!("[create-fix-pr] 無任何修補內容可 commit，流程中止。");
            return;
        }
        let _ = Command::new("git").args(["commit", "-m", "auto-fix: 自動修補 PR by bot"]).status();
        // 3. push 至 origin
        println!("[create-fix-pr] push 至 origin/{} ...", branch);
        let _ = Command::new("git").args(["push", "origin", &branch]).status();
        // 4. 檢查 gh CLI
        let gh_check = Command::new("gh").arg("--version").output();
        if gh_check.is_err() || !gh_check.as_ref().unwrap().status.success() {
            eprintln!("[create-fix-pr] gh CLI 未安裝或未在 PATH，請安裝 GitHub CLI (gh) 後再試。流程中止。");
            return;
        }
        // 5. 用 gh CLI 建 PR
        println!("[create-fix-pr] 建立 PR ...");
        let pr_body = "本 PR 由 bot 自動產生，包含自動修補 patch 及 KB 引用。\n\n- Patch: eva_auto_fix.patch\n- KB: [knowledge_base/error_log.md](./crates/eva-tools/knowledge_base/error_log.md)";
        let gh_status = Command::new("gh")
            .args(["pr", "create", "--base", "main", "--head", &branch, "--title", "[auto-fix] 自動修補 PR (by bot)", "--body", pr_body])
            .status();
        match gh_status {
            Ok(status) if status.success() => println!("[create-fix-pr] PR 建立成功。"),
            Ok(status) => eprintln!("[create-fix-pr] PR 建立失敗，狀態碼: {}", status),
            Err(e) => eprintln!("[create-fix-pr] PR 建立失敗: {} (請確認 gh CLI 與權限)", e),
        }
        return;
    }
    match cmd.as_str() {
        "test-record-error" => {
            // 測試寫入知識庫
            let res = crate::error_recovery::record_error(
                "test_module",
                "let x = None.unwrap();",
                "auto_fix",
                "fail"
            );
            match res {
                Ok(_) => println!("record_error() 測試成功，請檢查 knowledge_base/error_log.md"),
                Err(e) => eprintln!("record_error() 測試失敗: {}", e),
            }
        },
        "simple-demo" => {
            use std::process::Command;
            use regex::Regex;
            use std::fs;
            println!("[simple-demo] 執行 cargo check...");
            let output = Command::new("cargo")
                .arg("check")
                .output()
                .expect("failed to execute cargo check");
            let stderr = String::from_utf8_lossy(&output.stderr);
            let re = Regex::new(r"can't find crate for `([^`]+)`").unwrap();
            let mut fixed = false;
            for cap in re.captures_iter(&stderr) {
                let crate_name = &cap[1];
                println!("[simple-demo] 偵測到缺 crate: {}，自動補 Cargo.toml", crate_name);
                let cargo_toml = fs::read_to_string("Cargo.toml").expect("讀取 Cargo.toml 失敗");
                if !cargo_toml.contains(&format!("{} =", crate_name)) {
                    let new_content = cargo_toml.replace("[dependencies]", &format!("[dependencies]\n{} = \"*\"", crate_name));
                    fs::write("Cargo.toml", new_content).expect("寫入 Cargo.toml 失敗");
                    fixed = true;
                }
                let _ = crate::error_recovery::record_error(
                    "simple-demo",
                    &format!("can't find crate for `{}`", crate_name),
                    "auto-fix",
                    "fixed"
                );
            }
            if fixed {
                println!("[simple-demo] 已自動補依賴，請重新執行 cargo check 驗證。");
            } else {
                println!("[simple-demo] 未偵測到缺 crate 錯誤。");
            }
        },
        "check-structure" => run_check_structure(),
        "auto-generate" => run_auto_generate(),
        "auto-loop" => {
            match crate::auto_loop_executor::run_auto_loop_executor() {
                Ok(_) => println!("[auto-loop] 自動修復流程結束。"),
                Err(e) => eprintln!("[auto-loop] 執行失敗: {}", e),
            }
        },
        "api" => {
            println!("啟動 EVA HTTP API 伺服器於 http://127.0.0.1:8080 ...");
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(eva_api_server());
        }
        _ => {
            eprintln!("未知指令: {}", cmd);
            std::process::exit(1);
        }
    }
}

// async 啟動 API Server
async fn eva_api_server() {
    // 注意：請確保已正確 use crate::api_server
    crate::api_server::serve("127.0.0.1:8080").await;
}

fn run_check_structure() {
    use check_structure::{parse_expected_paths, scan_existing_rs, diff_missing};
    let expected = parse_expected_paths("ARCHITECTURE/ARCHITECTURE_TREE.md");
    let actual = scan_existing_rs(Path::new("."));
    let missing = diff_missing(&expected, &actual);
    if missing.is_empty() {
        println!("✅ 結構檢查通過，無缺漏檔案");
    } else {
        println!("⚠️ 缺少以下檔案:");
        for p in missing {
            println!(" - {}", p.display());
        }
    }
}
