//! eva-core: Skeleton for IntentListener, TaskPlanner, PlanExecutor

pub mod intent_listener;
pub mod task_planner;
pub mod plan_executor;
pub mod listener;
pub mod memory;
pub mod memory_vector;

use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;
use chrono::Utc;

pub struct EvaCore;

impl EvaCore {
    pub async fn run() -> anyhow::Result<()> {
        // channel: WS / STDIN 皆送這裡
        let (intent_tx, mut intent_rx) = mpsc::channel::<String>(16);

        // 1️⃣ WebSocket listener
        tokio::spawn(crate::listener::ws::start(intent_tx.clone()));

        // 2️⃣ CLI fallback（可保留原 stdin 版）
        tokio::spawn(async move {
            use tokio::io::{self, AsyncBufReadExt};
            let mut lines = io::BufReader::new(io::stdin()).lines();
            while let Ok(Some(l)) = lines.next_line().await {
                intent_tx.send(l).await.ok();
            }
        });

        // 3️⃣ 處理意圖
        while let Some(intent) = intent_rx.recv().await {
            info!("EvaCore got intent: {intent}");
            // 將使用者意圖寫入短期記憶
            crate::memory::append(crate::memory::MemoryRecord {
                trace_id: Uuid::new_v4().to_string(),
                role: "user".into(),
                text: intent,
                ts: Utc::now(),
            });
            // TODO: call planner / executor
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        EvaCore::run();
    }
}
