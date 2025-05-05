//! 掃描 src/ 與 crates/，比對 ARCHITECTURE.md 標題行，列出遺漏檔案。
use std::fs;
use std::path::{Path, PathBuf};

/// 解析 ARCHITECTURE.md，取得預期路徑集合
pub fn parse_expected_paths(md_path: &str) -> Vec<PathBuf> {
    let content = fs::read_to_string(md_path).expect("讀取 ARCHITECTURE.md 失敗");
    let mut paths = Vec::new();
    for line in content.lines() {
        if line.trim_start().starts_with("src/") || line.trim_start().starts_with("crates/") {
            let raw = line.trim().trim_matches('`');
            if !raw.is_empty() {
                paths.push(PathBuf::from(raw));
            }
        }
    }
    paths
}

/// 列出實際存在的檔案路徑
pub fn scan_existing_rs(root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if root.is_dir() {
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
    expected.iter().filter(|e| !actual.contains(e)).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_expected_paths() {
        let paths = parse_expected_paths("ARCHITECTURE.md");
        assert!(paths.iter().any(|p| p.to_string_lossy().contains("src/app/cli.rs")));
    }
}
