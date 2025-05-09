use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("check-layout") => {
            if let Err(e) = check_layout() {
                eprintln!("[xtask] layout check failed: {}", e);
                std::process::exit(1);
            }
        },
        Some("gen-spec-hash") => {
            if let Err(e) = xtask::spec_hash::write_spec_hash() {
                eprintln!("[xtask] gen-spec-hash failed: {}", e);
                std::process::exit(1);
            }
        },
        Some("gen-status-table") => {
            xtask::status::gen_status_table();
            return;
        },
        _ => {
            eprintln!("Usage: cargo xtask [check-layout|gen-spec-hash|gen-status-table]");
            std::process::exit(1);
        }
    }
}

fn check_layout() -> Result<(), String> {
    let tree_path = Path::new("ARCHITECTURE/ARCHITECTURE_TREE_CANONICAL.md");
    let tree = fs::read_to_string(tree_path).map_err(|e| format!("read tree: {}", e))?;
    let mut expected = vec![];
    for line in tree.lines() {
        if let Some(stripped) = line.strip_prefix("- ") {
            let path = stripped.trim().split_whitespace().next().unwrap_or("");
            if !path.is_empty() && !path.starts_with("#") {
                expected.push(path.replace("/", std::path::MAIN_SEPARATOR_STR));
            }
        }
    }
    let mut actual = vec![];
    for entry in walkdir::WalkDir::new(".") {
        let entry = entry.map_err(|e| format!("walkdir: {}", e))?;
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(".").unwrap().to_string_lossy().to_string();
            actual.push(rel);
        }
    }
    expected.sort();
    actual.sort();
    let missing: Vec<_> = expected.iter().filter(|p| !actual.contains(p)).collect();
    let extra: Vec<_> = actual.iter().filter(|p| !expected.contains(p)).collect();
    if !missing.is_empty() {
        eprintln!("[xtask] 缺少檔案：\n{}", missing.join("\n"));
    }
    if !extra.is_empty() {
        eprintln!("[xtask] 多餘檔案：\n{}", extra.join("\n"));
    }
    if !missing.is_empty() || !extra.is_empty() {
        return Err("結構不符 canonical tree".into());
    }
    println!("[xtask] 結構檢查通過");
    Ok(())
}
