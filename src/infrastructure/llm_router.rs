//! LLM Router - 自動切換本地 llama.cpp 與雲端 OpenAI，並支援增強學習 (feedback 收集)
//!
//! 本模組實現:
//! 1. `LlmProvider` trait: 抽象化 LLM 推論介面
//! 2. `LlmRouter`: 根據設定自動選擇本地或雲端 LLM，並在本地失敗/信心不足時 fallback 雲端
//! 3. `record_feedback`: 收集用戶回饋資料，方便後續增量微調或 RAG 知識庫建立
//!
//! 注意: 目前 LocalLlmAdapter 的推論仍為 TODO (placeholder)。後續可整合 `llama_cpp_rs` 並加入 GPU 參數。

use std::{fs::{self, OpenOptions}, io::Write, path::PathBuf};

use super::{local_llm_adapter::LocalLlmAdapter, openai_client::OpenAiClient};

/// 抽象化 LLM 推論介面
pub trait LlmProvider {
    /// 輸入 prompt，回傳模型輸出或錯誤字串
    fn infer(&self, prompt: &str) -> Result<String, String>;
}

// 為兩個實作體實現 Trait
impl LlmProvider for LocalLlmAdapter {
    fn infer(&self, prompt: &str) -> Result<String, String> {
        LocalLlmAdapter::infer(self, prompt)
    }
}

impl LlmProvider for OpenAiClient {
    fn infer(&self, prompt: &str) -> Result<String, String> {
        OpenAiClient::infer(self, prompt)
    }
}

/// LLM Router: 根據設定決定使用本地或雲端 LLM，並可收集回饋
pub struct LlmRouter {
    exec: Option<LocalLlmAdapter>,
    cloud: Option<OpenAiClient>,
    /// 是否優先使用本地 LLM
    prefer_local: bool,
    /// 儲存回饋資料的路徑 (jsonl)
    feedback_path: PathBuf,
}

impl LlmRouter {
    pub fn new(prefer_local: bool, enable_cloud: bool) -> Self {
        let feedback_path = PathBuf::from("data/feedback.jsonl");
        if let Some(dir) = feedback_path.parent() {
            if let Err(e) = fs::create_dir_all(dir) {
                eprintln!("[LlmRouter] 建立資料夾失敗: {}", e);
            }
        }
        Self {
            exec: Some(LocalLlmAdapter::new()),
            cloud: if enable_cloud { OpenAiClient::try_new() } else { None },
            prefer_local,
            feedback_path,
        }
    }

    /// 核心推論 API。根據設定嘗試本地→雲端 fallback。
    pub fn infer(&self, prompt: &str) -> Result<String, String> {
        // helper closure
        let try_infer = |provider: &dyn LlmProvider| provider.infer(prompt);

        // order depends on prefer_local flag
        if self.prefer_local {
            // 1. exe (llama.cpp)
            if let Some(exec) = &self.exec {
                match try_infer(exec) {
                    Ok(ans) => return Ok(ans),
                    Err(e) => {
                        eprintln!("[LlmRouter] llama.exe 失敗: {}", e);
                        // 嘗試雲端 fallback，但如果 cloud provider 沒有 API key，直接回傳錯誤，不 panic
                        if let Some(cloud) = &self.cloud {
                            match try_infer(cloud) {
                                Ok(ans) => return Ok(ans),
                                Err(e2) => return Err(format!("本地 LLM 失敗: {}\n雲端 LLM 也失敗: {}", e, e2)),
                            }
                        } else {
                            return Err(format!("本地 LLM 失敗: {}\n未設定雲端 LLM (無 API KEY)", e));
                        }
                    },
                }
            } else {
                return Err("本地 LLM 未初始化".into());
            }
        } else {
            // cloud first
            if let Some(cloud) = &self.cloud {
                if let Ok(ans) = try_infer(cloud) {
                    return Ok(ans);
                }
            }
            // exe
            if let Some(exec) = &self.exec {
                match try_infer(exec) {
                    Ok(ans) => return Ok(ans),
                    Err(e) => return Err(format!("雲端 LLM 失敗，本地 LLM 也失敗: {}", e)),
                }
            }
        }
        Err("無可用的 LLM provider".to_string())
    }

    /// 收集用戶回饋 (good / bad) 以便微調或 RAG
    pub fn record_feedback(&self, prompt: &str, answer: &str, good: bool) {
        let record = serde_json::json!({
            "prompt": prompt,
            "answer": answer,
            "good": good,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&self.feedback_path) {
            if let Err(e) = writeln!(file, "{}", record) {
                eprintln!("[LlmRouter] 寫入回饋失敗: {}", e);
            }
        }
    }
}
