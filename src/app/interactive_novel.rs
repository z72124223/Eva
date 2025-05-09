pub fn run() {
    use std::io::{self, Write};

    fn random_start_menu() {
        println!("\n=== 隨機開始 ===");
        // 進入時自動呼叫 LLM 生成故事開場
        let story = match crate::infrastructure::llm_router::LlmRouter::new(true, true)
            .infer("請生成一段科幻冒險小說的故事開場，語氣要有懸念和臨場感，50字以內。") {
            Ok(s) => s,
            Err(e) => format!("[錯誤] LLM 生成失敗：{}", e),
        };
        println!("\n{}\n", story);
        println!("--------------------");
        println!("1 ) 返回上一頁");
        print!("\n請輸入指令（或直接 Enter 返回）：");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        // 直接返回上一層，不做其它處理
    }

    fn custom_game_menu() {
        loop {
            println!("\n=== 自訂遊戲 ===");
            println!("1 ) 與 AI 討論故事/角色卡/曲線");
            println!("2 ) 返回上一頁");
            print!("請輸入選項：");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            match input.trim() {
                "1" => {
                    println!("\n[自訂遊戲] 讓我們先聊聊你想要的故事… (輸入 done 結束)");
                    let mut story_notes = Vec::new();
                    loop {
                        print!("> ");
                        io::stdout().flush().unwrap();
                        let mut detail = String::new();
                        if io::stdin().read_line(&mut detail).is_err() { break; }
                        let txt = detail.trim();
                        if txt.eq_ignore_ascii_case("done") { break; }
                        story_notes.push(txt.to_string());
                    }
                    println!("\n[摘要] 你提供的故事需求：");
                    for (i, note) in story_notes.iter().enumerate() {
                        println!("{}. {}", i + 1, note);
                    }
                    println!("\n(stub) 未來將根據這些需求生成角色卡與故事曲線 …");
                    println!("(按 Enter 返回上一層)");
                    let _ = io::stdin().read_line(&mut String::new());
                },
                "2" => break,
                _ => println!("無效選項，請重試。"),
            }
        }
    }

    loop {
        println!("\n=== 互動小說 ===");
        println!("1 ) 開始新遊戲");
        println!("2 ) 載入進度");
        println!("3 ) 返回上一頁");
        print!("請輸入選項：");
        io::stdout().flush().ok();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        match buf.trim() {
            "1" => {
                // 新遊戲子選單
                loop {
                    println!("\n=== 開始新遊戲 ===");
                    println!("1 ) 隨機開始");
                    println!("2 ) 自訂遊戲");
                    println!("3 ) 返回上一頁");
                    print!("請輸入選項：");
                    io::stdout().flush().ok();
                    let mut sub = String::new();
                    io::stdin().read_line(&mut sub).ok();
                    match sub.trim() {
                        "1" => random_start_menu(),
                        "2" => custom_game_menu(),
                        "3" => break,
                        _ => println!("無效選項，請重試。"),
                    }
                }
            },
            "2" => {
                println!("[stub] 載入進度 ... (未來會實作存檔)");
            },
            "3" => {
                println!("返回主選單");
                break;
            },
            _ => println!("無效選項，請重試。"),
        }
    }
}
