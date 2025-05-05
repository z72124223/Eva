//! context_builder.rs
//! 收集缺檔、TODO 區段與藍圖片段，組成 LLM prompt
use std::fs;
use std::path::Path;

pub struct PromptContext {
    pub missing_files: Vec<String>,
    pub todos: Vec<(String, usize)>, // (檔案路徑, 行號)
    pub blueprint_excerpt: String,
}

impl PromptContext {
    /// 讀取 ARCHITECTURE.md 前 200 行作為 blueprint_excerpt
    pub fn from_blueprint(md_path: &str) -> String {
        fs::read_to_string(md_path)
            .map(|s| s.lines().take(200).collect::<Vec<_>>().join("\n"))
            .unwrap_or_default()
    }
    /// 掃描 src/ 及 crates/，收集 TODO 區段
    pub fn find_todos(root: &Path) -> Vec<(String, usize)> {
        let mut result = Vec::new();
        if root.is_dir() {
            for entry in fs::read_dir(root).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    result.extend(Self::find_todos(&path));
                } else if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        if let Ok(content) = fs::read_to_string(&path) {
                            for (i, line) in content.lines().enumerate() {
                                if line.contains("TODO") {
                                    result.push((path.display().to_string(), i + 1));
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }
    /// 建立 PromptContext
    pub fn build(missing_files: Vec<String>, todos: Vec<(String, usize)>, blueprint_excerpt: String) -> Self {
        Self { missing_files, todos, blueprint_excerpt }
    }
}
