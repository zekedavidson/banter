/// WebSocket upgrade route handler.
///
/// Accepts WS upgrade requests at `/api/v1/ws` and hands them off
/// to the per-connection handler.

use axum::extract::{State, WebSocketUpgrade};
use axum::response::Response;

use crate::AppState;
use super::connection;

/// GET /api/v1/ws — WebSocket upgrade
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(move |socket| connection::handle_connection(socket, state))
}
