//! 根據 ExpectedPath 建立 .rs 檔，並自動更新父 mod.rs。
use std::fs;
use std::path::PathBuf;

/// 建立缺漏的 .rs 檔案
#[allow(dead_code)]
pub fn create_missing_rs(paths: &[PathBuf]) {
    for path in paths {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        if !path.exists() {
            fs::write(path, "// auto-generated\n").ok();
            // 同步更新父 mod.rs
            if let Some(parent) = path.parent() {
                let mod_path = parent.join("mod.rs");
                let fname = path.file_stem().unwrap().to_string_lossy();
                let mod_decl = format!("pub mod {};", fname);
                if mod_path.exists() {
                    let old = fs::read_to_string(&mod_path).unwrap();
                    if !old.contains(&mod_decl) {
                        let mut new = old;
                        new.push_str("\n");
                        new.push_str(&mod_decl);
                        fs::write(&mod_path, new).ok();
                    }
                } else {
                    fs::write(&mod_path, format!("{}\n", mod_decl)).ok();
                }
            }
        }
    }
}
