use axum::{
    extract::WebSocketUpgrade,
    routing::get,
    Router,
};

use futures_util::StreamExt;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;
use tracing::info;
use eva_ops::{WS_CONN_TOTAL, BUS_MESSAGES_TOTAL};
use jsonschema::{JSONSchema, Draft};
use once_cell::sync::Lazy;
use serde_json::json;
use uuid::Uuid;

// 預先編譯 Intent JSON schema（draft-07）
static SCHEMA: Lazy<JSONSchema> = Lazy::new(|| {
    // schema 置於 crate 根目錄 intent_schema.json
    let raw = include_str!("../../intent_schema.json");
    let schema_json: serde_json::Value = serde_json::from_str(raw).expect("schema json");
    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_json)
        .expect("compile schema")
});

pub async fn start(ws_tx: Sender<String>) -> anyhow::Result<()> {
    let app = Router::new().route("/ws", get(move |ws: WebSocketUpgrade| {
        let tx = ws_tx.clone();
        async move {
            ws.on_upgrade(move |mut socket| async move {
                info!("🟢 client connected");
                WS_CONN_TOTAL.inc();
                while let Some(Ok(msg)) = socket.next().await {
                    if let axum::extract::ws::Message::Text(text) = msg {
                        // 包裝成 Bus JSON 物件
                        let bus_msg = json!({
                            "role": "user",
                            "type": "intent",
                            "trace_id": Uuid::new_v4().to_string(),
                            "payload": { "text": text }
                        });

                        // schema 驗證
                        if let Err(validation_errors) = SCHEMA.validate(&bus_msg) {
                            let errs: Vec<_> = validation_errors.collect();
                            tracing::warn!("invalid message: {:?}", errs);
                            continue;
                        }

                        // 發送至核心 channel
                        if tx.send(bus_msg.to_string()).await.is_ok() {
                            BUS_MESSAGES_TOTAL.inc();
                        }

                        // echo OK
                        socket
                            .send(axum::extract::ws::Message::Text("ACK".into()))
                            .await
                            .ok();
                    }
                }
                info!("🔴 client disconnected");
            })
        }
    }));

    let addr: SocketAddr = "0.0.0.0:9898".parse()?;
    info!("WebSocket listener on ws://{addr}/ws");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
