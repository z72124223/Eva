pub fn is_locked(path: &str, content: &str) -> bool {
    // 鎖檔條件：檔名符合且內容含鎖檔標籤
    path.ends_with("ARCHITECTURE_TREE.md") && content.contains("EVA-GUARD: LOCKED_FILE")
}
