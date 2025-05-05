//! api_server.rs
//! EVA API Server (Axum)
//! - /v1/infer   → forward 到 openai_client
//! - /v1/upgrade → forward 到 module_generator
//! - 預留 SSE/WS、JWT、流控、metrics

use axum::{routing::post, Router, Json, extract::State, http::Request, middleware::{self, Next}, response::Response, routing::get};
use axum::extract::TypedHeader;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;
use tower::{BoxError, limit::RateLimitLayer};
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
}


#[derive(Deserialize)]
pub struct InferRequest {
    pub prompt: String,
}

#[derive(Serialize)]
pub struct InferResponse {
    pub result: String,
}

#[derive(Deserialize)]
pub struct UpgradeRequest {
    pub action: String,
}

#[derive(Serialize)]
pub struct UpgradeResponse {
    pub status: String,
    pub detail: String,
}

async fn infer_handler(Json(payload): Json<InferRequest>) -> Json<InferResponse> {
    // TODO: 呼叫 openai_client
    let resp = crate::infrastructure::openai_client::OpenAiClient::new().infer(&payload.prompt);
    let result = match resp {
        Ok(r) => r,
        Err(e) => format!("[OpenAI Error] {}", e),
    };
    Json(InferResponse { result })
}

async fn upgrade_handler(Json(payload): Json<UpgradeRequest>) -> Json<UpgradeResponse> {
    use crate::crates::eva_tools::src::check_structure;
    use crate::crates::eva_tools::src::module_generator;
    use std::path::Path;
    // 解析 action，這裡假設 action = "auto"
    let mut msg = String::new();
    if payload.action == "auto" {
        let expected = check_structure::parse_expected_paths("ARCHITECTURE.md");
        let mut actual = check_structure::scan_existing_rs(Path::new("../../src"));
        actual.extend(check_structure::scan_existing_rs(Path::new("../../crates")));
        let missing = check_structure::diff_missing(&expected, &actual);
        if missing.is_empty() {
            msg = "所有架構檔案皆齊全，不需補檔。".to_string();
        } else {
            module_generator::create_missing_rs(&missing);
            msg = format!("已補齊缺漏: {:?}", missing.iter().map(|p| p.display().to_string()).collect::<Vec<_>>());
        }
    } else {
        msg = format!("未知 action: {} (stub)", payload.action);
    }
    Json(UpgradeResponse {
        status: "ok".into(),
        detail: msg,
    })
}

async fn jwt_auth<B>(State(state): State<Arc<AppState>>, req: Request<B>, next: Next<B>) -> Result<Response, axum::http::StatusCode> {
    use axum::http::StatusCode;
    let auth = req.headers().get("authorization").and_then(|v| v.to_str().ok());
    if let Some(auth) = auth {
        if let Some(token) = auth.strip_prefix("Bearer ") {
            // 驗證 JWT（這裡簡化，僅比對 secret，相容未來擴充）
            if token == state.jwt_secret {
                return Ok(next.run(req).await);
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

async fn metrics_handler() -> String {
    prometheus_exporter::encode_http_response().unwrap_or_else(|_| "# metrics unavailable".into())
}

pub async fn serve(addr: &str) {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "testsecret".to_string());
    let state = Arc::new(AppState { jwt_secret });
    // 啟動 prometheus_exporter
    static START: std::sync::Once = std::sync::Once::new();
    START.call_once(|| {
        let _ = prometheus_exporter::start("127.0.0.1:9898");
    });
    let app = Router::new()
        .route("/v1/infer", post(infer_handler))
        .route("/v1/upgrade", post(upgrade_handler))
        .route("/metrics", get(metrics_handler))
        .layer(middleware::from_fn_with_state(state.clone(), jwt_auth))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(RateLimitLayer::new(2, Duration::from_secs(1))) // 每秒最多2請求
                .layer(RequestBodyLimitLayer::new(1024 * 32)) // 請求體最大32KB
                .layer(TraceLayer::new_for_http())
        );
    let addr: SocketAddr = addr.parse().unwrap();
    println!("EVA API Server 啟動於 http://{} (JWT+QPS保護+metrics)", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
