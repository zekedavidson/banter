use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// PostgreSQL enum: member_role
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "member_role", rename_all = "lowercase")]
pub enum MemberRole {
    Owner,
    Admin,
    Member,
}

/// Mirrors public.servers table
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub banner_url: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub owner_id: Uuid,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
}

/// Request body for creating a server
#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub color: Option<String>,
}

/// Mirrors public.server_members table
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerMember {
    pub server_id: Uuid,
    pub user_id: Uuid,
    pub role: MemberRole,
    pub joined_at: DateTime<Utc>,
}

/// Server with member count (for discovery)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerWithMemberCount {
    #[serde(flatten)]
    pub server: Server,
    pub member_count: i64,
}
