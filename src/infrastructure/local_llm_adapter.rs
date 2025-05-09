//! 本地 LLM (llama.cpp) 適配器，支援 GPU

use std::path::{Path, PathBuf};
use std::fs;

/// 預設掃描的本地模型目錄 (Windows)
const DEFAULT_MODEL_DIR: &str = "C:/llama/models";

/// 支援的模型副檔名 (gguf / bin)
const MODEL_EXTS: &[&str] = &["gguf", "bin", "ggml"];

/// 嘗試在目錄底下尋找第一個符合副檔名的模型檔
fn find_model_file<P: AsRef<Path>>(dir: P) -> Option<PathBuf> {
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if MODEL_EXTS.iter().any(|&m| m.eq_ignore_ascii_case(ext)) {
                return Some(path);
            }
        }
    }
    None
}

#[derive(Clone, Debug)]
pub struct LocalLlmAdapter {
    pub model_path: Option<PathBuf>,
}

impl LocalLlmAdapter {
    /// 偵測本地模型是否存在
    pub fn detect() -> bool {
        let model_dir = std::env::var("LLAMA_MODEL_DIR").unwrap_or_else(|_| DEFAULT_MODEL_DIR.to_string());
        find_model_file(&model_dir).is_some()
    }
    /// 建立新的 Adapter，會嘗試自動尋找本地 quantized gguf/bin 模型
    pub fn new() -> Self {
        let model_dir = std::env::var("LLAMA_MODEL_DIR").unwrap_or_else(|_| DEFAULT_MODEL_DIR.to_string());
        let model_path = find_model_file(&model_dir);
        if let Some(ref p) = model_path {
            println!("[LocalLlmAdapter] 偵測到本地模型: {}", p.display());
        } else {
            eprintln!("[LocalLlmAdapter] 未找到本地模型於 {}，後續推論將回傳 stub", model_dir);
        }
        Self { model_path }
    }

    /// 目前僅回傳 stub，待整合 `llama_cpp_rs`
    pub fn infer(&self, prompt: &str) -> Result<String, String> {
        if let Some(p) = &self.model_path {
            // 呼叫 llama-simple.exe 進行推論（GPU 加速由 dll 處理）
            use std::process::Command;
            // 嘗試多種可執行檔名稱 (官方 build)
            let exe_candidates = [
                "C:/llama/main.exe",
                "C:/llama/llama.exe",
                "C:/llama/llama-run.exe",
                "C:/llama/llama-cli.exe",
                "C:/llama/llama-simple.exe",
            ];
            let exe = exe_candidates
                .iter()
                .find(|p| std::path::Path::new(p).exists())
                .unwrap_or(&"C:/llama/main.exe");
            let model_path = &p.to_string_lossy();
            // llama-run.exe 參數格式: --ngl 999 <model_path> <prompt>
            let args = ["--ngl", "999", model_path.as_ref(), prompt];
            println!("[LocalLlmAdapter] 呼叫: {} {:?}", exe, args);
            if !std::path::Path::new(exe).exists() {
                eprintln!("[LocalLlmAdapter] 找不到可執行檔，請確認 C:/llama/ 安裝完整");
                return Err("找不到可執行檔，請確認 C:/llama/ 安裝完整".into());
            }
            let output = match Command::new(exe).args(&args).output() {
                Ok(out) => out,
                Err(e) => {
                    eprintln!("[LocalLlmAdapter] 執行失敗: {}", e);
                    return Err(format!("執行失敗: {}", e));
                }
            };

            println!("[LocalLlmAdapter] {} exit code: {:?}", exe, output.status);
            println!("[LocalLlmAdapter] stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("[LocalLlmAdapter] stderr:\n{}", String::from_utf8_lossy(&output.stderr));

            if !output.status.success() {
                return Err(format!(
                    "{} 返回錯誤碼 {}: {}",
                    exe, output.status,
                    String::from_utf8_lossy(&output.stderr)
                ));
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            // 取最後一行作為回應
            if let Some(last) = stdout.lines().last() {
                Ok(last.trim().to_string())
            } else {
                Ok(stdout.trim().to_string())
            }
        } else {
            Ok(format!("[no-local-model] {}", prompt))
        }
    }
}

// 保留舊版介面 (deprecate)
#[allow(dead_code)]
pub fn legacy_new() -> LocalLlmAdapter {
    LocalLlmAdapter::new()
}
