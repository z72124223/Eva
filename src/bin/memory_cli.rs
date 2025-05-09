use eva::services::memory_store::{Memory, MemoryQuery, MemoryStore};
use eva::services::memory_store_sqlite::SqliteMemoryStore;
use chrono::Utc;
use std::env;

fn main() {
    let db_path = env::var("EVA_MEMORY_DB").unwrap_or_else(|_| "eva_memory.sqlite3".to_string());
    let store = SqliteMemoryStore::new(&db_path).expect("DB 初始化失敗");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("\n用法: memory_cli <add|get|del> [參數]\n");
        eprintln!("  add <user_id> <type> <content> [tag1,tag2,...]");
        eprintln!("  get <user_id> [type] [tag1,tag2,...]");
        eprintln!("  del <id>\n");
        return;
    }
    match args[1].as_str() {
        "add" => {
            if args.len() < 5 {
                eprintln!("參數不足: add <user_id> <type> <content> [tag1,tag2,...]");
                return;
            }
            let tags = if args.len() > 5 {
                args[5].split(',').map(|s| s.trim().to_string()).collect()
            } else { vec![] };
            let mem = Memory {
                id: 0,
                owner_id: args[2].clone(), // 以 user_id 作為 owner_id
                user_id: args[2].clone(),
                memory_type: args[3].clone(),
                content: args[4].clone(),
                created_at: Utc::now(),
                scope: "ai-visible".to_string(), // 預設給 ai-visible
                tags,
            };
            let id = store.add_memory(&mem).expect("新增失敗");
            println!("已新增記憶 id={}", id);
        },
        "get" => {
            let user_id = args.get(2).cloned();
            let memory_type = args.get(3).cloned();
            let tags = args.get(4).map(|s| s.split(',').map(|x| x.trim().to_string()).collect());
            let q = MemoryQuery {
                user_id,
                memory_type,
                tags,
                since: None,
                until: None,
                limit: Some(20),
            };
            let ms = store.get_memories(q).expect("查詢失敗");
            for m in ms {
                println!("[{}] {} [{}] {} | {}", m.id, m.user_id, m.memory_type, m.content, m.tags.join(","));
            }
        },
        "del" => {
            if args.len() < 3 {
                eprintln!("參數不足: del <id>");
                return;
            }
            let id: i64 = args[2].parse().expect("id 必須為數字");
            store.delete_memory(id).expect("刪除失敗");
            println!("已刪除記憶 id={}", id);
        },
        _ => {
            eprintln!("未知指令: {}", args[1]);
        }
    }
}
