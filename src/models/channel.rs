use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// PostgreSQL enum: channel_type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "channel_type", rename_all = "lowercase")]
pub enum ChannelType {
    Text,
    Voice,
}

/// Mirrors public.channels table
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Channel {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    pub kind: ChannelType,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

/// Request body for creating a channel
#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub kind: ChannelType,
}

/// Mirrors public.voice_states table
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct VoiceState {
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub muted: bool,
    pub video_on: bool,
    pub joined_at: DateTime<Utc>,
}
