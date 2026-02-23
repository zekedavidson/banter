/// DM REST handlers: list DM channels, create/find DM, get DM messages

use axum::extract::{Path, State, Query};
use axum::Json;
use uuid::Uuid;

use crate::AppState;
use crate::auth::AuthUser;
use crate::error::AppResult;
use crate::models::{
    DmChannel, CreateDmRequest, DmChannelSummary, DmMessageWithAuthor,
    MessageQuery, ProfileSummary,
};

/// GET /api/v1/dms — list DM channels for the authenticated user
pub async fn list_dms(
    auth: AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<DmChannelSummary>>> {
    // Get all DM channels the user is part of, with the other user's profile
    // and last message info
    let rows = sqlx::query_as::<_, DmSummaryRow>(
        r#"
        SELECT
            dc.id as dm_channel_id,
            p.id as other_user_id,
            p.username as other_username,
            p.display_name as other_display_name,
            p.avatar_url as other_avatar_url,
            (
                SELECT content FROM dm_messages
                WHERE dm_channel_id = dc.id
                ORDER BY created_at DESC LIMIT 1
            ) as last_message,
            (
                SELECT created_at FROM dm_messages
                WHERE dm_channel_id = dc.id
                ORDER BY created_at DESC LIMIT 1
            ) as last_message_at
        FROM dm_channels dc
        INNER JOIN dm_members my_m ON my_m.dm_channel_id = dc.id AND my_m.user_id = $1
        INNER JOIN dm_members other_m ON other_m.dm_channel_id = dc.id AND other_m.user_id != $1
        INNER JOIN profiles p ON p.id = other_m.user_id
        ORDER BY last_message_at DESC NULLS LAST
        "#,
    )
    .bind(auth.user_id)
    .fetch_all(&state.pool)
    .await?;

    let summaries = rows
        .into_iter()
        .map(|r| DmChannelSummary {
            id: r.dm_channel_id,
            other_user: ProfileSummary {
                id: r.other_user_id,
                username: r.other_username,
                display_name: r.other_display_name,
                avatar_url: r.other_avatar_url,
            },
            last_message: r.last_message,
            last_message_at: r.last_message_at,
            unread_count: 0, // TODO: implement unread tracking
        })
        .collect();

    Ok(Json(summaries))
}

#[derive(sqlx::FromRow)]
struct DmSummaryRow {
    dm_channel_id: Uuid,
    other_user_id: Uuid,
    other_username: Option<String>,
    other_display_name: String,
    other_avatar_url: Option<String>,
    last_message: Option<String>,
    last_message_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// POST /api/v1/dms — find or create a DM channel with another user
pub async fn create_dm(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateDmRequest>,
) -> AppResult<Json<DmChannel>> {
    // Check if a DM channel already exists between us
    let existing = sqlx::query_as::<_, DmChannel>(
        r#"
        SELECT dc.* FROM dm_channels dc
        INNER JOIN dm_members m1 ON m1.dm_channel_id = dc.id AND m1.user_id = $1
        INNER JOIN dm_members m2 ON m2.dm_channel_id = dc.id AND m2.user_id = $2
        LIMIT 1
        "#,
    )
    .bind(auth.user_id)
    .bind(body.target_user_id)
    .fetch_optional(&state.pool)
    .await?;

    if let Some(dm) = existing {
        return Ok(Json(dm));
    }

    // Create new DM channel
    let mut tx = state.pool.begin().await?;

    let dm = sqlx::query_as::<_, DmChannel>(
        "INSERT INTO dm_channels DEFAULT VALUES RETURNING *"
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("INSERT INTO dm_members (dm_channel_id, user_id) VALUES ($1, $2), ($1, $3)")
        .bind(dm.id)
        .bind(auth.user_id)
        .bind(body.target_user_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(Json(dm))
}

#[derive(sqlx::FromRow)]
struct DmMessageRow {
    id: Uuid,
    dm_channel_id: Uuid,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
    author_id: Uuid,
    author_username: Option<String>,
    author_display_name: String,
    author_avatar_url: Option<String>,
}

/// GET /api/v1/dms/:id/messages?before=&limit=
pub async fn get_dm_messages(
    _auth: AuthUser,
    State(state): State<AppState>,
    Path(dm_channel_id): Path<Uuid>,
    Query(q): Query<MessageQuery>,
) -> AppResult<Json<Vec<DmMessageWithAuthor>>> {
    let limit = q.limit.unwrap_or(50).min(100);

    let rows = if let Some(before_id) = q.before {
        sqlx::query_as::<_, DmMessageRow>(
            r#"
            SELECT
                m.id, m.dm_channel_id, m.content, m.created_at,
                p.id as author_id, p.username as author_username,
                p.display_name as author_display_name,
                p.avatar_url as author_avatar_url
            FROM dm_messages m
            INNER JOIN profiles p ON p.id = m.author_id
            WHERE m.dm_channel_id = $1
              AND m.created_at < (SELECT created_at FROM dm_messages WHERE id = $2)
            ORDER BY m.created_at DESC
            LIMIT $3
            "#,
        )
        .bind(dm_channel_id)
        .bind(before_id)
        .bind(limit)
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as::<_, DmMessageRow>(
            r#"
            SELECT
                m.id, m.dm_channel_id, m.content, m.created_at,
                p.id as author_id, p.username as author_username,
                p.display_name as author_display_name,
                p.avatar_url as author_avatar_url
            FROM dm_messages m
            INNER JOIN profiles p ON p.id = m.author_id
            WHERE m.dm_channel_id = $1
            ORDER BY m.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(dm_channel_id)
        .bind(limit)
        .fetch_all(&state.pool)
        .await?
    };

    let messages = rows
        .into_iter()
        .map(|r| DmMessageWithAuthor {
            id: r.id,
            dm_channel_id: r.dm_channel_id,
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
