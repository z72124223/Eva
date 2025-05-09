pub fn is_patch_safe(patch: &str, max_size: usize, danger_keywords: &[&str]) -> Result<(), String> {
    if patch.len() > max_size {
        return Err(format!("patch size {} > MAX_PATCH_SIZE {}", patch.len(), max_size));
    }
    for &kw in danger_keywords {
        if patch.contains(kw) {
            return Err(format!("patch contains dangerous keyword: {}", kw));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_patch_too_large() {
        let patch = "a".repeat(2048);
        assert!(is_patch_safe(&patch, 1024, &["unsafe"]).is_err());
    }
    #[test]
    fn test_patch_contains_unsafe() {
        let patch = "let x = unsafe { 1 };";
        assert!(is_patch_safe(patch, 1024, &["unsafe"]).is_err());
    }
    #[test]
    fn test_patch_contains_path_traversal() {
        let patch = "../../etc/passwd";
        assert!(is_patch_safe(patch, 1024, &["../"]).is_err());
    }
    #[test]
    fn test_patch_safe_normal() {
        let patch = "let x = 1 + 2;";
        assert!(is_patch_safe(patch, 1024, &["unsafe"]).is_ok());
    }
}
