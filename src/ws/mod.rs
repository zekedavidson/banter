pub mod events;
pub mod connection;
pub mod router;

use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use self::events::WsEvent;

/// Shared WebSocket state for real-time event routing.
///
/// - `channel_senders`: per-channel broadcast senders for fan-out
/// - `user_connections`: per-user list of mpsc senders (supports multi-device)
#[derive(Clone)]
pub struct WsState {
    inner: Arc<WsStateInner>,
}

struct WsStateInner {
    pub channel_senders: DashMap<Uuid, broadcast::Sender<WsEvent>>,
    pub user_connections: DashMap<Uuid, Vec<mpsc::UnboundedSender<WsEvent>>>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(WsStateInner {
                channel_senders: DashMap::new(),
                user_connections: DashMap::new(),
            }),
        }
    }

    /// Get or create a broadcast sender for a channel.
    pub fn get_or_create_channel(&self, channel_id: Uuid) -> broadcast::Sender<WsEvent> {
        self.inner
            .channel_senders
            .entry(channel_id)
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(256);
                tx
            })
            .clone()
    }

    /// Register a user connection (returns an mpsc receiver for outbound events).
    pub fn register_user(&self, user_id: Uuid) -> mpsc::UnboundedReceiver<WsEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.inner
            .user_connections
            .entry(user_id)
            .or_default()
            .push(tx);
        rx
    }

    /// Remove a user's connection sender.
    pub fn unregister_user(&self, user_id: Uuid, tx: &mpsc::UnboundedSender<WsEvent>) {
        if let Some(mut senders) = self.inner.user_connections.get_mut(&user_id) {
            senders.retain(|s| !s.same_channel(tx));
            if senders.is_empty() {
                drop(senders);
                self.inner.user_connections.remove(&user_id);
            }
        }
    }

    /// Send an event to a specific user (all their connected devices).
    pub fn send_to_user(&self, user_id: &Uuid, event: &WsEvent) {
        if let Some(senders) = self.inner.user_connections.get(user_id) {
            for tx in senders.iter() {
                let _ = tx.send(event.clone());
            }
        }
    }

    /// Broadcast an event to all subscribers of a channel.
    pub fn broadcast_to_channel(&self, channel_id: &Uuid, event: WsEvent) {
        if let Some(tx) = self.inner.channel_senders.get(channel_id) {
            let _ = tx.send(event);
        }
    }
}
