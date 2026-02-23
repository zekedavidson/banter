use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// PostgreSQL enum: user_status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Online,
    Idle,
    Dnd,
    Offline,
}

/// Mirrors public.profiles table (linked to Supabase auth.users)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Profile {
    pub id: Uuid,
    pub username: Option<String>,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Subset used for message/event author info
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProfileSummary {
    pub id: Uuid,
    pub username: Option<String>,
    pub display_name: String,
    pub avatar_url: Option<String>,
}
