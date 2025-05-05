//! Prompt Adapter，統一 LLM 介面

pub enum LlmProvider {
    OpenAi,
    Local,
}

pub struct PromptAdapter {
    openai: crate::infrastructure::openai_client::OpenAiClient,
    local: crate::infrastructure::local_llm_adapter::LocalLlmAdapter,
}

impl PromptAdapter {
    pub fn new() -> Self {
        Self {
            openai: crate::infrastructure::openai_client::OpenAiClient::new(),
            local: crate::infrastructure::local_llm_adapter::LocalLlmAdapter::new(),
        }
    }
    pub fn infer(&self, prompt: &str, provider: LlmProvider) -> String {
        match provider {
            LlmProvider::OpenAi => self.openai.infer(prompt),
            LlmProvider::Local => self.local.infer(prompt),
        }
    }
}
