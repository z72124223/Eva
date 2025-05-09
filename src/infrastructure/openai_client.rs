//! OpenAI LLM Client
//! 從環境變數讀取 API Key，呼叫雲端 OpenAI API
// 移除未使用的 std::env 匯入

pub struct OpenAiClient {
    api_key: String,
}

impl OpenAiClient {
    pub fn try_new() -> Option<Self> {
        std::env::var("OPENAI_API_KEY").ok().map(|key| Self { api_key: key })
    }

    pub fn infer(&self, prompt: &str) -> Result<String, String> {
        use std::{thread, time::Duration};
        let client = match reqwest::blocking::Client::builder().timeout(Duration::from_secs(15)).build() {
            Ok(c) => c,
            Err(e) => return Err(format!("reqwest client 構建失敗: {}", e)),
        };
        let mut retries = 0;
        let max_retries = 3;
        let mut wait = 2;
        loop {
            let res = client.post("https://api.openai.com/v1/chat/completions")
                .bearer_auth(&self.api_key)
                .header("Content-Type", "application/json")
                .body(format!(
                    "{{\"model\":\"gpt-3.5-turbo\",\"messages\":[{{\"role\":\"user\",\"content\":\"{}\"}}]}}",
                    prompt.replace('"', "\\\"")
                ))
                .send();
            match res {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let json: serde_json::Value = match resp.json() {
                            Ok(j) => j,
                            Err(e) => return Err(format!("回應解析失敗: {}", e)),
                        };
                        if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
                            return Ok(content.trim().to_string());
                        } else {
                            return Err(format!("OpenAI 回應格式錯誤: {:?}", json));
                        }
                    } else {
                        let code = resp.status();
                        let err = resp.text().unwrap_or_default();
                        if retries < max_retries {
                            retries += 1;
                            thread::sleep(Duration::from_secs(wait));
                            wait *= 2;
                            continue;
                        }
                        return Err(format!("OpenAI API 錯誤: {} {}", code, err));
                    }
                },
                Err(e) => {
                    if retries < max_retries {
                        retries += 1;
                        thread::sleep(Duration::from_secs(wait));
                        wait *= 2;
                        continue;
                    }
                    return Err(format!("HTTP 請求失敗: {}", e));
                }
            }
        }
    }
}
