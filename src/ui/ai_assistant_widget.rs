use std::io::{self, Write};

/// 全域 AI 助手 Widget
/// 顯示一次 AI 助手輸入框，回傳是否輸入 quit
pub fn one_shot() -> bool {
    println!("\n=== 🤖 AI 助手 (全域) ===");
    print!("你想問什麼？(輸入 quit 結束) > ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_ok() {
        let q = buf.trim();
        if q.eq_ignore_ascii_case("quit") {
            return true;
        }
        if !q.is_empty() {
            // 呼叫 OpenAI LLM（只有設置 API_KEY 時才執行）
            if let Some(cli) = crate::infrastructure::openai_client::OpenAiClient::try_new() {
                match cli.infer(q) {
                    Ok(ans) => println!("AI 回答：{}", ans),
                    Err(e) => println!("[OpenAI 錯誤] {}", e),
                }
            } else {
                println!("未設定 OPENAI_API_KEY，無法使用雲端 AI");
            }
        }
    }
    false
}
