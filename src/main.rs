//! Banter Backend — A highly concurrent Rust server for the bubbly-chat Discord clone.
//!
//! Powered by Axum, Tokio, SQLx (Supabase PostgreSQL), and LiveKit.

use axum::routing::{delete, get, patch, post};
use axum::Router;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

mod config;
mod db;
mod error;
mod auth;
mod models;
mod handlers;
mod ws;

/// Shared application state available to all handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: config::AppConfig,
    pub ws_state: ws::WsState,
}

/// Build the `/api/v1` router with all REST + WS routes.
fn api_router() -> Router<AppState> {
    Router::new()
        // Auth (combined on same path)
        .route("/auth/me", get(auth::handlers::get_me).patch(auth::handlers::update_me))
        // Servers
        .route("/servers", get(handlers::servers::list_servers).post(handlers::servers::create_server))
        .route("/servers/discover", get(handlers::servers::discover_servers))
        .route("/servers/:id", get(handlers::servers::get_server))
        .route("/servers/:id/join", post(handlers::servers::join_server))
        .route("/servers/:id/leave", delete(handlers::servers::leave_server))
        // Channels
        .route("/servers/:id/channels", get(handlers::channels::list_channels).post(handlers::channels::create_channel))
        .route("/channels/:id/messages", get(handlers::channels::get_messages))
        .route("/channels/:id/voice-state", get(handlers::channels::get_voice_state))
        // DMs
        .route("/dms", get(handlers::dms::list_dms).post(handlers::dms::create_dm))
        .route("/dms/:id/messages", get(handlers::dms::get_dm_messages))
        // Voice / LiveKit
        .route("/voice/token", post(handlers::voice::generate_voice_token))
        // WebSocket
        .route("/ws", get(ws::router::ws_handler))
}

#[tokio::main]
async fn main() {
    // Load .env file (ignoring errors if not present)
    dotenvy::dotenv().ok();

    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,banter_backend=debug")),
        )
        .init();

    // Load configuration
    let config = config::AppConfig::from_env();
    let port = config.backend_port;

    // Create database connection pool
    tracing::info!("Connecting to Supabase PostgreSQL...");
    let pool = db::create_pool(&config.database_url).await;
    tracing::info!("Database connected ✓");

    // Build shared state
    let state = AppState {
        pool,
        config,
        ws_state: ws::WsState::new(),
    };

    // Build application
    let app = Router::new()
        .nest("/api/v1", api_router())
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind port");

    tracing::info!("🚀 Banter backend listening on http://{addr}");
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
