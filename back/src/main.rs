use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

async fn health_check() -> &'static str {
    "ok"
}

async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello from Axum + Tokio!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    // Logging / tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Router với 2 route
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/hello", get(hello).post(hello));

    // Chạy server trên 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server chạy tại http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
