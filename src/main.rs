use anyhow::Context;
use askama::Template;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use std::net::SocketAddr;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

use tracing::info;

#[derive(Debug, Clone)]
pub struct AppState {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let port = dotenvy::var("PORT").map_or_else(|_| Ok(3001), |p| p.parse::<u16>())?;
    let state = AppState {};

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/favicon.ico", ServeFile::new("assets/favicon.ico"))
        .layer(CompressionLayer::new())
        .with_state(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    if let Ok(addr) = listener.local_addr() {
        info!("server started at {}", addr);
    }

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .context("failed to start server")
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

async fn health() -> (StatusCode, impl IntoResponse) {
    (StatusCode::OK, "OK")
}
