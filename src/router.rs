#[allow(dead_code, unused)]
use crate::config::Config;
use crate::entities::presets;
use crate::error::internal_error;

use anyhow::Ok;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;

// Struct to hold the application state
pub struct AppState {
    pub pool: PgPool,
    pub config: &'static Config,
}

pub async fn serve(config: &'static Config, db: PgPool) -> Result<(), anyhow::Error> {
    let app_state = AppState { pool: db, config };
    let socket_address = build_address(config);

    let app = Router::new()
        // .merge(comments::router())
        .merge(presets::router())
        .layer(Extension(Arc::new(app_state)))
        .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();
    axum::serve(listener, app)
        .await
        .map_err(internal_error)
        .unwrap();

    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}

fn build_address(config: &'static Config) -> SocketAddr {
    let host = config.server_host();
    let port = config.server_port();

    let address = format!("{}:{}", host, port);

    address.parse().unwrap()
}
