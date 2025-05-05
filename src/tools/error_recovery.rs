use std::fs::OpenOptions;
use std::io::Write;

pub fn record_error(module: &str, snippet: &str, action: &str, status: &str) -> std::io::Result<()> {
    use chrono::Utc;
    let line = format!(
        "{} | {} | {} | {} | {}\n",
        Utc::now().to_rfc3339(), module, snippet, action, status
    );
    OpenOptions::new()
        .append(true)
        .create(true)
        .open("knowledge_base/error_log.md")?
        .write_all(line.as_bytes())
}
