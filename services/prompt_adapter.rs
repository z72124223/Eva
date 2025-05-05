//! prompt_adapter.rs
//! LLM 統一調用介面，支援 OpenAI 與 Local LLM
use crate::infrastructure::openai_client::OpenAiClient;

pub enum LlmProvider {
    OpenAi,
    Local,
}

pub struct PromptAdapter {
    openai: OpenAiClient,
    // local: Option<LocalLlmAdapter>, // 可擴充
}

impl PromptAdapter {
    pub fn new() -> Self {
        Self {
            openai: OpenAiClient::new(),
            // local: None,
        }
    }
    pub fn infer(&self, prompt: &str, provider: LlmProvider) -> String {
        match provider {
            LlmProvider::OpenAi => {
                match self.openai.infer(prompt) {
                    Ok(resp) => resp,
                    Err(e) => format!("// OpenAI 產生失敗: {}\n// TODO: {}", e, prompt),
                }
            }
            LlmProvider::Local => {
                "// Local LLM 尚未實作\n".into()
            }
        }
    }
}
