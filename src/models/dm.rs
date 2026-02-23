use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProfileSummary;

/// Mirrors public.dm_channels table
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DmChannel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Mirrors public.dm_members table
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DmMember {
    pub dm_channel_id: Uuid,
    pub user_id: Uuid,
}

/// Mirrors public.dm_messages table
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DmMessage {
    pub id: Uuid,
    pub dm_channel_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// DM channel summary for sidebar (with other participant + last message)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmChannelSummary {
    pub id: Uuid,
    pub other_user: ProfileSummary,
    pub last_message: Option<String>,
    pub last_message_at: Option<DateTime<Utc>>,
    pub unread_count: i64,
}

/// DM message with embedded author (for API responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmMessageWithAuthor {
    pub id: Uuid,
    pub dm_channel_id: Uuid,
    pub author: ProfileSummary,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// Request body for creating / finding a DM channel
#[derive(Debug, Deserialize)]
pub struct CreateDmRequest {
    pub target_user_id: Uuid,
}
