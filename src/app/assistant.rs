pub fn run() {
    use std::io::{self, Write};

    println!("🛠️ 助手模式：輸入問題 (quit 結束)");
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        line.clear();
        if io::stdin().read_line(&mut line).is_err() {
            println!("讀取失敗，返回主選單");
            break;
        }
        let q = line.trim();
        if q.eq_ignore_ascii_case("quit") {
            break;
        }
        // 簡易回顯，未串接 LLM
        println!("(stub) 回答：{}", q);
    }
}
