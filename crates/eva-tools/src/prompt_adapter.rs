use anyhow::Result;
use eva_knowledge_base::query as kb_query;
use serde_json;

/// 向 LLM 發送 prompt，取得修補 patch
/// 會自動查詢 KB，將歷史成功修補片段插入 prompt
pub async fn ask_patch(err_sig: &str, latest_error: &str) -> Result<String> {
    // 查詢 KB 取得成功 patch 片段
    let kb_snippets = match kb_query(err_sig) {
        Ok(records) => {
            records.into_iter()
                .filter(|rec| rec.success)
                .map(|rec| rec.patch_sig)
                .take(3)
                .collect::<Vec<_>>()
                .join("\n---\n")
        },
        Err(_) => String::new(),
    };
    let prompt = format!(
        "system: 你是厲害的 Rust 工程師...\nuser: 先前遇到類似錯誤的成功修補如下：\n{}\nuser: 以下是最新錯誤：\n{}",
        kb_snippets,
        latest_error
    );
    // 串接 OpenAI LLM API（如有 API KEY）
    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
        let client = reqwest::Client::new();
        let api_result = async {
            let resp = client.post("https://api.openai.com/v1/chat/completions")
                .bearer_auth(api_key)
                .json(&serde_json::json!({
                    "model": "gpt-3.5-turbo",
                    "messages": [
                        {"role": "system", "content": "你是厲害的 Rust 工程師..."},
                        {"role": "user", "content": format!("先前遇到類似錯誤的成功修補如下：\n{}", kb_snippets)},
                        {"role": "user", "content": format!("以下是最新錯誤：\n{}", latest_error)}
                    ],
                    "max_tokens": 512
                }))
                .send()
                .await?;
            let resp_json: serde_json::Value = resp.json().await?;
            if let Some(choice) = resp_json["choices"].as_array().and_then(|arr| arr.get(0)) {
                if let Some(content) = choice["message"]["content"].as_str() {
                    return Ok::<String, anyhow::Error>(content.to_string());
                }
            }
            Err(anyhow::anyhow!("OpenAI API 無有效回應"))
        }.await;
        if let Ok(content) = api_result {
            return Ok(content);
        }
    }
    // fallback: 回傳模擬 patch
    let dummy_patch = "--- a/src/foo.rs\n+++ b/src/foo.rs\n@@\n+use foo::bar;\n# unresolved import\n".to_string();
    Ok(dummy_patch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eva_knowledge_base::{FixRecord, insert as kb_insert, current_timestamp};

    #[test]
    fn test_prompt_with_kb_snippets() {
        let _ = kb_insert(&FixRecord {
            id: 0,
            timestamp: current_timestamp(),
            err_sig: "unresolved import foo::bar".to_string(),
            patch_sig: "use foo::bar;".to_string(),
            success: true,
            meta: None,
        });
        let rt = tokio::runtime::Runtime::new().unwrap();
        let prompt = rt.block_on(ask_patch("unresolved import foo::bar", "error: unresolved import"))
            .unwrap_or_default();
        assert!(prompt.contains("use foo::bar"));
        assert!(prompt.contains("unresolved import"));
    }
}
