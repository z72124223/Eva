//! 本地 LLM (llama_cpp_rs) 原生 GPU 適配器
//!
//! 透過 `llama_cpp_rs` 載入 GGUF 模型並於單一 process 內推論。
//! - 優先使用 CUDA backend (feature: cuda)
//! - 若初始化失敗，Caller 應退回其他 provider
//!
//! 目前僅實作最簡問答 (`n_predict` 固定 128)。後續可加參數。

use std::{path::PathBuf, sync::OnceLock};
use llama_cpp_rs as llama;
use llama::{LLama};

/// 全域模型快取，避免重複載入佔用 VRAM。
static GLOBAL_MODEL: OnceLock<LLama> = OnceLock::new();

pub struct LocalLlmNativeAdapter {
    model_path: PathBuf,
}

impl LocalLlmNativeAdapter {
    pub fn new(model_path: &str) -> Self {
        Self { model_path: PathBuf::from(model_path) }
    }

    /// 初始化全域模型，之後重複呼叫不會重新載入。
    fn ensure_model(&self) -> Result<&LLama, String> {
        GLOBAL_MODEL.get_or_try_init(|| {
            llama::load_model(self.model_path.to_string_lossy().as_ref())
                .map_err(|e| format!("load_model failed: {:?}", e))
        })
    }

    pub fn infer(&self, prompt: &str) -> Result<String, String> {
        let model = self.ensure_model()?;
        let params = llama::LlamaPredictParams::default().n_predict(128);
        match llama::llama_predict(model, prompt, &params) {
            Ok(out) => Ok(out.trim().to_string()),
            Err(e) => Err(format!("llama_predict error: {:?}", e)),
        }
    }
}
