use std::io::{self, Write};
use crate::app::mode_manifest;

pub fn show() -> Option<usize> {
    println!("\n=== EVA 主選單 ===");
    for m in mode_manifest::list() {
        println!("{} ) {}", m.id, m.name);
    }
    println!("0 ) 🚪 離開");
    print!("請輸入選項：");
    io::stdout().flush().ok()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok()?;
    buf.trim().parse::<usize>().ok()
}
