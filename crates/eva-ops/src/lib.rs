//! eva-ops — Prometheus metrics & health ping

use std::time::Duration;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// ───────────────────────────────────────────────────────────
// ⬇ 新增：Prometheus metrics 與 healthchecks.io ping

use axum::{routing::get, Router};
use once_cell::sync::Lazy;
use prometheus::{Encoder, Registry, TextEncoder, IntCounter, Opts};

use std::net::SocketAddr;
use tokio::time::sleep;

static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);
static EVA_OPS_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    let opts = Opts::new("eva_ops_requests_total", "Example counter for eva-ops");
    let counter = IntCounter::with_opts(opts).unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

pub static WS_CONN_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    let counter = IntCounter::new("eva_ops_ws_connections_total", "累計 WS 連線").unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

pub static BUS_MESSAGES_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    let counter = IntCounter::new("eva_bus_messages_total", "累計 Bus 訊息").unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

/// 啟動 `/metrics` HTTP 服務與定時 health ping
pub async fn init_metrics() -> anyhow::Result<()> {
    // ── 1. 5 分鐘一次的 healthchecks.io ping ───────────────────
    tokio::spawn(async {
        const URL: &str = "https://hc-ping.com/1ac5f660-3873-462b-8102-e4090e1a791a";
        loop {
            if let Err(e) = reqwest::get(URL).await {
                tracing::warn!("health ping failed: {:?}", e);
            }
            sleep(Duration::from_secs(300)).await;
        }
    });

    // ── 2. /metrics HTTP 端點 ────────────────────────────────────
    let app = Router::new().route("/metrics", get(metrics_handler));
    let addr: SocketAddr = "0.0.0.0:8081".parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("metrics server listening on 0.0.0.0:8081");
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app.into_make_service()).await {
            tracing::warn!("metrics server error: {e:?}");
        }
    });

    Ok(())
}

async fn metrics_handler() -> String {
    EVA_OPS_COUNTER.inc(); // 每次有人存取 /metrics 就遞增一次
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    TextEncoder::new()
        .encode(&metric_families, &mut buffer)
        .expect("encode metrics");
    String::from_utf8(buffer).expect("metrics utf8")
}
