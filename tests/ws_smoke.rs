#[tokio::test]
async fn ws_smoke() {
    use tokio_tungstenite::{connect_async, tungstenite::Message};
    use futures_util::SinkExt;

    // 啟動 core in background
    tokio::spawn(async { eva_core::EvaCore::run().await.unwrap(); });

    // 連線
    let (mut ws, _) = connect_async("ws://127.0.0.1:9898/ws")
        .await
        .expect("connect");
    ws.send(Message::Text("ping".into())).await.ok();
}
