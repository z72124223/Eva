//! 掃描 src/ 與 crates/，比對 ARCHITECTURE.md 標題行，列出遺漏檔案。
use std::fs;
use std::path::{Path, PathBuf};

/// 解析 ARCHITECTURE.md，取得預期路徑集合
pub fn parse_expected_paths(md_path: &str) -> Vec<PathBuf> {
    use regex::Regex;
    let content = fs::read_to_string(md_path).expect("讀取 ARCHITECTURE.md 失敗");
    let mut paths = Vec::new();
    let mut in_canonical = false;
    let mut found = false;
    let re = Regex::new(r"^\s*([\w./-]+)\s*(?:#\s*illustrative)?$").unwrap();
    for line in content.lines() {
        if line.contains("tree # canonical") { in_canonical = true; found = true; continue; }
        if in_canonical && line.starts_with("```") { break; }
        if !in_canonical { continue; }
        if let Some(caps) = re.captures(line) {
            let raw = caps.get(1).unwrap().as_str();
            // 跳過只有 src/ 或 crates/ 單行
            if raw == "src/" || raw == "crates/" { continue; }
            // 跳過 illustrative
            if line.contains("# illustrative") { continue; }
            paths.push(PathBuf::from(raw));
        }
    }
    // fallback: 舊流程
    if !found {
        for line in content.lines() {
            if line.trim_start().starts_with("src/") || line.trim_start().starts_with("crates/") {
                let raw = line.trim().trim_matches('`');
                if !raw.is_empty() && raw != "src/" && raw != "crates/" {
                    paths.push(PathBuf::from(raw));
                }
            }
        }
    }
    paths
}


/// 列出實際存在的檔案路徑
pub fn scan_existing_rs(root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if root.is_dir() {
        // 先把資料夾本身加入 result
        result.push(root.to_path_buf());
        for entry in fs::read_dir(root).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                result.extend(scan_existing_rs(&path));
            } else if let Some(ext) = path.extension() {
                if ext == "rs" { result.push(path); }
            }
        }
    }
    result
}


/// 比對預期與實際，回傳缺漏清單
pub fn diff_missing(expected: &[PathBuf], actual: &[PathBuf]) -> Vec<PathBuf> {
    let norm = |p: &PathBuf| {
        let s = p.to_string_lossy();
        s.trim_end_matches(|c| c == '/' || c == '\\').to_string()
    };
    expected
        .iter()
        .filter(|e| {
            let en = norm(e);
            !actual.iter().any(|a| {
                let an = norm(a);
                an == en || an.ends_with(&en)
            })
        })
        .cloned()
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_expected_paths() {
        use std::path::PathBuf;
        let md_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../ARCHITECTURE/ARCHITECTURE_TREE.md");
        let paths = parse_expected_paths(md_path.to_str().unwrap());
        assert!(!paths.is_empty(), "ARCHITECTURE_TREE.md 應該至少解析到一條路徑");
    }
}
