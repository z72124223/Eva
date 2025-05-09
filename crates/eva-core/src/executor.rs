//! PlanExecutor + 懶載 LLM adapter

use std::path::PathBuf;

pub enum LlmAdapter {
    OpenAi(super::super::super::src::infrastructure::openai_client::OpenAiClient),
    Local(super::super::super::src::infrastructure::local_llm_adapter::LocalLlmAdapter),
}

pub struct PlanExecutor {
    pub adapter: LlmAdapter,
}

impl PlanExecutor {
    /// 根據 API key/local model 狀態初始化 LLM adapter
    pub fn try_new() -> Result<Self, String> {
        if let Some(client) = super::super::super::src::infrastructure::openai_client::OpenAiClient::try_new() {
            return Ok(PlanExecutor { adapter: LlmAdapter::OpenAi(client) });
        }
        let local = super::super::super::src::infrastructure::local_llm_adapter::LocalLlmAdapter::new();
        if local.model_path.is_some() {
            return Ok(PlanExecutor { adapter: LlmAdapter::Local(local) });
        }
        Err("未偵測到 OpenAI API key，也沒有本地模型，無法執行 LLM 任務".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;

    fn set_api_key(val: Option<&str>) {
        if let Some(v) = val {
            env::set_var("OPENAI_API_KEY", v);
        } else {
            env::remove_var("OPENAI_API_KEY");
        }
    }
    fn create_fake_model(dir: &str) -> PathBuf {
        let _ = fs::create_dir_all(dir);
        let fake = Path::new(dir).join("fake.gguf");
        fs::write(&fake, b"stub").unwrap();
        fake
    }
    fn remove_fake_model(dir: &str) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_openai_only() {
        set_api_key(Some("fake-key"));
        remove_fake_model("C:/llama/models");
        let exec = PlanExecutor::try_new();
        assert!(matches!(exec, Ok(PlanExecutor { adapter: LlmAdapter::OpenAi(_) })));
        set_api_key(None);
    }

    #[test]
    fn test_local_only() {
        set_api_key(None);
        let _ = create_fake_model("C:/llama/models");
        let exec = PlanExecutor::try_new();
        assert!(matches!(exec, Ok(PlanExecutor { adapter: LlmAdapter::Local(_) })));
        remove_fake_model("C:/llama/models");
    }

    #[test]
    fn test_both() {
        set_api_key(Some("fake-key"));
        let _ = create_fake_model("C:/llama/models");
        let exec = PlanExecutor::try_new();
        // 應優先 OpenAI
        assert!(matches!(exec, Ok(PlanExecutor { adapter: LlmAdapter::OpenAi(_) })));
        set_api_key(None);
        remove_fake_model("C:/llama/models");
    }

    #[test]
    fn test_none() {
        set_api_key(None);
        remove_fake_model("C:/llama/models");
        let exec = PlanExecutor::try_new();
        assert!(exec.is_err());
    }
}
