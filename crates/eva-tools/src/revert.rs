//! eva revert <tag> 回滾指令
use std::process::Command;
use std::io;

pub fn revert_to_tag(tag: &str) -> io::Result<()> {
    // 僅允許格式 eva-fail-YYYYMMDD-HHMMSS
    if !tag.starts_with("eva-fail-") {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "tag 格式錯誤，必須以 eva-fail- 開頭"));
    }
    // 先檢查 tag 是否存在
    let tag_check = Command::new("git").args(["rev-parse", "--verify", tag]).output()?;
    if !tag_check.status.success() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("找不到 tag: {}", tag)));
    }
    // 執行 git reset --hard <tag>
    let status = Command::new("git").args(["reset", "--hard", tag]).status()?;
    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, format!("git reset 失敗: {}", tag)));
    }
    println!("[eva revert] 已回滾至 tag: {}", tag);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    use std::process::Command;

    #[test]
    fn test_revert_to_tag_invalid_format() {
        let r = revert_to_tag("badtag");
        assert!(r.is_err());
    }

    // 集成測試需在 git repo 下運行，這裡僅驗證流程不 panic
    #[test]
    fn test_revert_to_tag_not_found() {
        let r = revert_to_tag("eva-fail-20990101-000000");
        assert!(r.is_err());
    }
}
