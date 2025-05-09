// 降級選單 UI，主程式僅呼叫 EVA 核心 IntentListener
// mod app;
// mod ui;
// mod tools;
// mod infrastructure;
// use std::path::Path;
// use eva_system_core::{init_self_inspect, get_project_summary};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    eva_ops::init_metrics().await?;
    // 啟動 EVA 核心 async 主流程
    eva_core::EvaCore::run().await?;
    Ok(())
}

