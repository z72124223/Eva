//! 本地 LLM (llama.cpp) 適配器，支援 GPU

pub struct LocalLlmAdapter;

impl LocalLlmAdapter {
    pub fn new() -> Self { Self }
    pub fn infer(&self, prompt: &str) -> String {
        // TODO: 實作 llama.cpp GPU 路徑呼叫
        format!("[llama.cpp 回應] {}", prompt)
    }
}
