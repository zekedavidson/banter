/// Full per-connection WebSocket handler.
///
/// Lifecycle:
///   1. Wait for `identify` event with JWT → verify → register user
///   2. Send `Ready` event
///   3. Enter main loop: read client events + forward outbound events
///   4. On disconnect: unregister user + broadcast presence offline

use std::collections::HashSet;

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::AppState;
use crate::auth::verify_token;
use crate::models::ProfileSummary;
use crate::ws::events::{ClientEvent, WsEvent};

/// Handle a single WebSocket connection from upgrade to close.
pub async fn handle_connection(socket: WebSocket, state: AppState) {
    let (mut ws_sink, mut ws_stream) = socket.split();

    // ── Step 1: Wait for identify ───────────────────────────────────
    let user_id = loop {
        match ws_stream.next().await {
            Some(Ok(Message::Text(text))) => {
                match serde_json::from_str::<ClientEvent>(&text) {
                    Ok(ClientEvent::Identify { token }) => {
                        match verify_token(&state.config.supabase_jwt_secret, &token) {
                            Ok(uid) => break uid,
                            Err(e) => {
                                let err = serde_json::to_string(&WsEvent::Error { message: e }).unwrap();
                                let _ = ws_sink.send(Message::Text(err)).await;
                                return; // close connection
                            }
                        }
                    }
                    _ => {
                        let err = serde_json::to_string(&WsEvent::Error {
                            message: "First message must be identify".into(),
                        }).unwrap();
                        let _ = ws_sink.send(Message::Text(err)).await;
                        return;
                    }
                }
            }
            Some(Ok(Message::Close(_))) | None => return,
            _ => continue, // skip pings, binary, etc.
        }
    };

    tracing::info!("WS identified: user={user_id}");

    // ── Step 2: Register user connection ────────────────────────────
    let mut user_rx = state.ws_state.register_user(user_id);

    // Send Ready event
    let ready = serde_json::to_string(&WsEvent::Ready { user_id }).unwrap();
    if ws_sink.send(Message::Text(ready)).await.is_err() {
        return;
    }

    // Broadcast presence: online
    broadcast_presence(&state, user_id, "online").await;

    // Track which channels/DMs this connection is subscribed to
    let mut subscribed_channels: HashSet<Uuid> = HashSet::new();
    let mut subscribed_dms: HashSet<Uuid> = HashSet::new();

    // Spawn a task to forward outbound events from the user's mpsc channel to the websocket
    let (outbound_tx, mut outbound_rx) = mpsc::unbounded_channel::<String>();
    let forward_task = tokio::spawn(async move {
        while let Some(json) = outbound_rx.recv().await {
            if ws_sink.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Spawn a task to forward broadcast channel events → user's outbound
    let outbound_tx_for_user = outbound_tx.clone();
    let user_event_task = tokio::spawn(async move {
        while let Some(event) = user_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&event) {
                if outbound_tx_for_user.send(json).is_err() {
                    break;
                }
            }
        }
    });

    // ── Step 3: Main read loop ──────────────────────────────────────
    while let Some(Ok(msg)) = ws_stream.next().await {
        match msg {
            Message::Text(text) => {
                let event = match serde_json::from_str::<ClientEvent>(&text) {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                handle_client_event(
                    &state,
                    user_id,
                    event,
                    &outbound_tx,
                    &mut subscribed_channels,
                    &mut subscribed_dms,
                ).await;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // ── Step 4: Cleanup on disconnect ───────────────────────────────
    tracing::info!("WS disconnected: user={user_id}");

    // Abort forwarding tasks
    forward_task.abort();
    user_event_task.abort();

    // Unsubscribe from all channels (drop broadcast receivers)
    // (receivers are dropped automatically when the tasks are aborted)

    // Unregister user connection
    // We need the original sender to identify which connection to remove.
    // Since we moved user_rx, we'll just remove all closed senders for this user.
    state.ws_state.cleanup_closed_senders(user_id);

    // Broadcast presence offline (only if no more connections for this user)
    if !state.ws_state.user_is_connected(&user_id) {
        broadcast_presence(&state, user_id, "offline").await;
    }
}

/// Process a single client event.
async fn handle_client_event(
    state: &AppState,
    user_id: Uuid,
    event: ClientEvent,
    outbound_tx: &mpsc::UnboundedSender<String>,
    subscribed_channels: &mut HashSet<Uuid>,
    subscribed_dms: &mut HashSet<Uuid>,
) {
    match event {
        ClientEvent::Identify { .. } => {
            // Already identified, ignore duplicate
        }

        ClientEvent::SubscribeChannel { channel_id } => {
            subscribed_channels.insert(channel_id);
            // Get or create a broadcast channel and spawn a receiver task
            let tx = state.ws_state.get_or_create_channel(channel_id);
            let mut rx = tx.subscribe();
            let outbound = outbound_tx.clone();
            tokio::spawn(async move {
                while let Ok(event) = rx.recv().await {
                    if let Ok(json) = serde_json::to_string(&event) {
                        if outbound.send(json).is_err() {
                            break;
                        }
                    }
                }
            });
            tracing::debug!("User {user_id} subscribed to channel {channel_id}");
        }

        ClientEvent::UnsubscribeChannel { channel_id } => {
            subscribed_channels.remove(&channel_id);
            // Broadcast receivers will be dropped when the spawn task ends
            tracing::debug!("User {user_id} unsubscribed from channel {channel_id}");
        }

        ClientEvent::SubscribeDm { dm_channel_id } => {
            subscribed_dms.insert(dm_channel_id);
            let tx = state.ws_state.get_or_create_channel(dm_channel_id);
            let mut rx = tx.subscribe();
            let outbound = outbound_tx.clone();
            tokio::spawn(async move {
                while let Ok(event) = rx.recv().await {
                    if let Ok(json) = serde_json::to_string(&event) {
                        if outbound.send(json).is_err() {
                            break;
                        }
                    }
                }
            });
            tracing::debug!("User {user_id} subscribed to DM {dm_channel_id}");
        }

        ClientEvent::UnsubscribeDm { dm_channel_id } => {
            subscribed_dms.remove(&dm_channel_id);
            tracing::debug!("User {user_id} unsubscribed from DM {dm_channel_id}");
        }

        ClientEvent::MessageCreate { channel_id, content } => {
            // 1. Persist message to DB
            let row = sqlx::query_as::<_, (Uuid, chrono::DateTime<chrono::Utc>)>(
                "INSERT INTO messages (channel_id, author_id, content) VALUES ($1, $2, $3) RETURNING id, created_at"
            )
            .bind(channel_id)
            .bind(user_id)
            .bind(&content)
            .fetch_one(&state.pool)
            .await;

            match row {
                Ok((msg_id, created_at)) => {
                    // 2. Fetch author profile summary
                    let author = get_profile_summary(&state, user_id).await;

                    // 3. Broadcast to channel
                    let event = WsEvent::MessageCreate {
                        id: msg_id,
                        channel_id,
                        author,
                        content,
                        created_at: created_at.to_rfc3339(),
                    };
                    state.ws_state.broadcast_to_channel(&channel_id, event);
                }
                Err(e) => {
                    tracing::error!("Failed to insert message: {e}");
                    let err = serde_json::to_string(&WsEvent::Error {
                        message: "Failed to send message".into(),
                    }).unwrap();
                    let _ = outbound_tx.send(err);
                }
            }
        }

        ClientEvent::DmCreate { dm_channel_id, content } => {
            // 1. Persist DM message to DB
            let row = sqlx::query_as::<_, (Uuid, chrono::DateTime<chrono::Utc>)>(
                "INSERT INTO dm_messages (dm_channel_id, author_id, content) VALUES ($1, $2, $3) RETURNING id, created_at"
            )
            .bind(dm_channel_id)
            .bind(user_id)
            .bind(&content)
            .fetch_one(&state.pool)
            .await;

            match row {
                Ok((msg_id, created_at)) => {
                    let author = get_profile_summary(&state, user_id).await;

                    // Broadcast to the DM channel (both participants receive it)
                    let event = WsEvent::DmCreate {
                        id: msg_id,
                        dm_channel_id,
                        author,
                        content,
                        created_at: created_at.to_rfc3339(),
                    };
                    state.ws_state.broadcast_to_channel(&dm_channel_id, event.clone());

                    // Also send directly to the other participant if they're connected
                    // (in case they haven't subscribed to this DM channel yet)
                    if let Ok(members) = sqlx::query_scalar::<_, Uuid>(
                        "SELECT user_id FROM dm_members WHERE dm_channel_id = $1 AND user_id != $2"
                    )
                    .bind(dm_channel_id)
                    .bind(user_id)
                    .fetch_all(&state.pool)
                    .await {
                        for member_id in members {
                            state.ws_state.send_to_user(&member_id, &event);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to insert DM: {e}");
                    let err = serde_json::to_string(&WsEvent::Error {
                        message: "Failed to send DM".into(),
                    }).unwrap();
                    let _ = outbound_tx.send(err);
                }
            }
        }

        ClientEvent::TypingStart { channel_id } => {
            let user = get_profile_summary(&state, user_id).await;
            let event = WsEvent::TypingStart { channel_id, user };
            state.ws_state.broadcast_to_channel(&channel_id, event);
        }

        ClientEvent::PresenceUpdate { status } => {
            // Update presence in DB
            let _ = sqlx::query("UPDATE profiles SET status = $1::user_status WHERE id = $2")
                .bind(&status)
                .bind(user_id)
                .execute(&state.pool)
                .await;

            // Broadcast to all servers the user belongs to
            broadcast_presence(state, user_id, &status).await;
        }
    }
}

/// Fetch a user's profile summary for embedding in events.
async fn get_profile_summary(state: &AppState, user_id: Uuid) -> ProfileSummary {
    sqlx::query_as::<_, ProfileSummary>(
        "SELECT id, username, display_name, avatar_url FROM profiles WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await
    .ok()
    .flatten()
    .unwrap_or_else(|| ProfileSummary {
        id: user_id,
        username: None,
        display_name: "Unknown".into(),
        avatar_url: None,
    })
}

/// Broadcast a presence update to users who share a server with this user.
async fn broadcast_presence(state: &AppState, user_id: Uuid, status: &str) {
    let event = WsEvent::PresenceUpdate {
        user_id,
        status: status.to_string(),
    };

    // Get all users who share a server with this user
    if let Ok(peer_ids) = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT DISTINCT sm2.user_id
        FROM server_members sm1
        JOIN server_members sm2 ON sm1.server_id = sm2.server_id
        WHERE sm1.user_id = $1 AND sm2.user_id != $1
        "#
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await {
        for peer_id in peer_ids {
            state.ws_state.send_to_user(&peer_id, &event);
        }
    }
}
