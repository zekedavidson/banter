/// Channel REST handlers: list channels, create channel, get messages

use axum::extract::{Path, State, Query};
use axum::Json;
use uuid::Uuid;

use crate::AppState;
use crate::auth::AuthUser;
use crate::error::{AppError, AppResult};
use crate::models::{
    Channel, CreateChannelRequest, MessageQuery,
    ProfileSummary, MessageWithAuthor, VoiceState,
};

/// GET /api/v1/servers/:id/channels
pub async fn list_channels(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(server_id): Path<Uuid>,
) -> AppResult<Json<Vec<Channel>>> {
    // Verify membership
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM server_members WHERE server_id = $1 AND user_id = $2)"
    )
    .bind(server_id)
    .bind(auth.user_id)
    .fetch_one(&state.pool)
    .await?;

    if !is_member {
        return Err(AppError::Forbidden("Not a member".into()));
    }

    let channels = sqlx::query_as::<_, Channel>(
        "SELECT * FROM channels WHERE server_id = $1 ORDER BY kind, position"
    )
    .bind(server_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(channels))
}

/// POST /api/v1/servers/:id/channels
pub async fn create_channel(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(server_id): Path<Uuid>,
    Json(body): Json<CreateChannelRequest>,
) -> AppResult<Json<Channel>> {
    // Only owner/admin can create channels (simplified: check membership for now)
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM server_members WHERE server_id = $1 AND user_id = $2)"
    )
    .bind(server_id)
    .bind(auth.user_id)
    .fetch_one(&state.pool)
    .await?;

    if !is_member {
        return Err(AppError::Forbidden("Not a member".into()));
    }

    let channel = sqlx::query_as::<_, Channel>(
        r#"
        INSERT INTO channels (server_id, name, kind)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(server_id)
    .bind(&body.name)
    .bind(&body.kind)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(channel))
}

/// Helper struct for the joined message + author query
#[derive(sqlx::FromRow)]
struct MessageRow {
    id: Uuid,
    channel_id: Uuid,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
    author_id: Uuid,
    author_username: Option<String>,
    author_display_name: String,
    author_avatar_url: Option<String>,
}

/// GET /api/v1/channels/:id/messages?before=&limit=
pub async fn get_messages(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
    Query(q): Query<MessageQuery>,
) -> AppResult<Json<Vec<MessageWithAuthor>>> {
    let limit = q.limit.unwrap_or(50).min(100);

    let rows = if let Some(before_id) = q.before {
        sqlx::query_as::<_, MessageRow>(
            r#"
            SELECT
                m.id, m.channel_id, m.content, m.created_at,
                p.id as author_id, p.username as author_username,
                p.display_name as author_display_name,
                p.avatar_url as author_avatar_url
            FROM messages m
            INNER JOIN profiles p ON p.id = m.author_id
            WHERE m.channel_id = $1
              AND m.created_at < (SELECT created_at FROM messages WHERE id = $2)
            ORDER BY m.created_at DESC
            LIMIT $3
            "#,
        )
        .bind(channel_id)
        .bind(before_id)
        .bind(limit)
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as::<_, MessageRow>(
            r#"
            SELECT
                m.id, m.channel_id, m.content, m.created_at,
                p.id as author_id, p.username as author_username,
                p.display_name as author_display_name,
                p.avatar_url as author_avatar_url
            FROM messages m
            INNER JOIN profiles p ON p.id = m.author_id
            WHERE m.channel_id = $1
            ORDER BY m.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(channel_id)
        .bind(limit)
        .fetch_all(&state.pool)
        .await?
    };

    let messages: Vec<MessageWithAuthor> = rows
        .into_iter()
        .map(|r| MessageWithAuthor {
            id: r.id,
            channel_id: r.channel_id,
            author: ProfileSummary {
                id: r.author_id,
                username: r.author_username,
                display_name: r.author_display_name,
                avatar_url: r.author_avatar_url,
            },
            content: r.content,
            created_at: r.created_at,
        })
        .collect();

    Ok(Json(messages))
}

/// GET /api/v1/channels/:id/voice-state
pub async fn get_voice_state(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
) -> AppResult<Json<Vec<VoiceState>>> {
    let states = sqlx::query_as::<_, VoiceState>(
        "SELECT * FROM voice_states WHERE channel_id = $1"
    )
    .bind(channel_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(states))
}
