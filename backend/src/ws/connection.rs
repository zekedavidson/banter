/// Per-connection WebSocket read/write loop.
///
/// This module will be fully implemented in Phase 3.
/// The structure is placed here so the project compiles.

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};

use crate::AppState;

/// Handle a single WebSocket connection.
///
/// Phase 3 will implement: identify handshake, subscribe/unsubscribe,
/// message routing, and presence tracking.
pub async fn handle_connection(socket: WebSocket, _state: AppState) {
    let (mut ws_sink, mut ws_stream) = socket.split();

    // Phase 3: Implement identify, event routing, etc.
    // For now, echo back any text messages as a smoke test.
    while let Some(Ok(msg)) = ws_stream.next().await {
        match msg {
            Message::Text(text) => {
                tracing::debug!("WS received: {text}");
                if ws_sink.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    tracing::debug!("WebSocket connection closed");
}
