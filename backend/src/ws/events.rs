/// WebSocket event types (client↔server protocol).
///
/// Uses serde's externally tagged enum for JSON serialization,
/// producing `{ "type": "message_create", ... }` shape.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::ProfileSummary;

/// Events sent from client → server
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientEvent {
    Identify { token: String },
    SubscribeChannel { channel_id: Uuid },
    UnsubscribeChannel { channel_id: Uuid },
    SubscribeDm { dm_channel_id: Uuid },
    UnsubscribeDm { dm_channel_id: Uuid },
    MessageCreate { channel_id: Uuid, content: String },
    DmCreate { dm_channel_id: Uuid, content: String },
    TypingStart { channel_id: Uuid },
    PresenceUpdate { status: String },
}

/// Events sent from server → client (also stored in broadcast channels)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsEvent {
    /// Server confirms successful identification
    Ready {
        user_id: Uuid,
    },

    /// New message in a text channel
    MessageCreate {
        id: Uuid,
        channel_id: Uuid,
        author: ProfileSummary,
        content: String,
        created_at: String,
    },

    /// New direct message
    DmCreate {
        id: Uuid,
        dm_channel_id: Uuid,
        author: ProfileSummary,
        content: String,
        created_at: String,
    },

    /// Someone started typing
    TypingStart {
        channel_id: Uuid,
        user: ProfileSummary,
    },

    /// User presence changed
    PresenceUpdate {
        user_id: Uuid,
        status: String,
    },

    /// Voice channel state changed
    VoiceStateUpdate {
        channel_id: Uuid,
        user: ProfileSummary,
        action: String, // "join" | "leave" | "mute" | "unmute" | "video_on" | "video_off"
    },

    /// Server membership events
    MemberJoin {
        server_id: Uuid,
        user: ProfileSummary,
    },
    MemberLeave {
        server_id: Uuid,
        user_id: Uuid,
    },

    /// Error message
    Error {
        message: String,
    },
}
