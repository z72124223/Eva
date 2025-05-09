use std::fs;
use std::path::Path;
use chrono::Utc;
use walkdir::WalkDir;

pub fn gen_status_table() {
    let today = Utc::now().format("%Y-%m-%d");
    let mut rows = vec![
        (1, "CI 覆蓋率門檻仍寫 70 % (CI_PR_AUTOMATION.md)", "自動修補機制風險 ≫70 % 的測試強度", if ci_threshold_updated() { "✅ 已調整" } else { "❌ 未調整" }),
        (2, "Router 拆分尚未落地（僅列進度表）", "仍是單點失效 + 循環依賴隱憂", "⏳ 未開始"),
        (3, "Spec 條款 ↔ 真實程式碼自動對映缺失", "文件漂移風險仍高", if has_check_layout() { "🔧 POC" } else { "❌ 缺失" }),
        (4, "資料隱私細節不足", "加解密／刪除權／角色隔離尚無實作方案", "❌ 缺失"),
        (5, "README 連結失准：指向 AUTO_FIX.md，實際檔不存在", "文件可信度下降，CI 產生死鏈", if auto_fix_exists() { "✅ 已修正" } else { "❌ 缺失" }),
        (6, "測試失敗熔斷 & 回滾策略 未見設計文／程式碼", "自動修補若連續失敗會瘋狂重試", "❌ 缺失"),
        (7, "Mutation testing 未引入", "難以驗證測試對錯誤分支的保護力", if mutants_in_ci() { "✅ 已引入" } else { "❌ 缺失" }),
        (8, "Ops 監控 / 降級機制仍缺", "服務上線後 SLO、告警、降級策略未落地", "❌ 缺失"),
    ];

    let mut md = format!("# 二、仍待處理的關鍵問題 ⚠️ ({})\n\n", today);
    md.push_str("| # | 問題 | 影響 | 最新狀態 |\n|---|------|------|-----------|\n");
    for (no, q, impact, status) in &rows {
        md.push_str(&format!("| {} | {} | {} | {} |\n", no, q, impact, status));
    }
    md.push_str("\n> 本表由 `cargo xtask gen-status-table` 自動生成\n");
    fs::write("CRITICAL_ISSUES_STATUS.md", md).expect("write status");
}

fn ci_threshold_updated() -> bool {
    let path = Path::new(".github/workflows/ci.yml");
    if let Ok(content) = fs::read_to_string(path) {
        content.contains("fail-under 95") && content.contains("fail-under 80")
    } else { false }
}

fn has_check_layout() -> bool {
    WalkDir::new("xtask/src").into_iter().any(|e| {
        e.ok().map(|e| e.file_name() == "check_layout.rs").unwrap_or(false)
    })
}

fn auto_fix_exists() -> bool {
    Path::new("ARCHITECTURE/AUTO_FIX.md").exists()
}

fn mutants_in_ci() -> bool {
    let path = Path::new(".github/workflows/ci.yml");
    if let Ok(content) = fs::read_to_string(path) {
        content.contains("cargo mutants")
    } else { false }
}
