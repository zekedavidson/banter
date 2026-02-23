/// Server REST handlers: list, create, discover, get, join, leave

use axum::extract::{Path, State, Query};
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use crate::auth::AuthUser;
use crate::error::{AppError, AppResult};
use crate::models::{Server, ServerMember, CreateServerRequest};

/// GET /api/v1/servers — user's joined servers
pub async fn list_servers(
    auth: AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Server>>> {
    let servers = sqlx::query_as::<_, Server>(
        r#"
        SELECT s.* FROM servers s
        INNER JOIN server_members sm ON sm.server_id = s.id
        WHERE sm.user_id = $1
        ORDER BY s.created_at
        "#,
    )
    .bind(auth.user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(servers))
}

/// POST /api/v1/servers — create a new server
pub async fn create_server(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<CreateServerRequest>,
) -> AppResult<Json<Server>> {
    let mut tx = state.pool.begin().await?;

    // Create the server
    let server = sqlx::query_as::<_, Server>(
        r#"
        INSERT INTO servers (name, icon, description, category, color, owner_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&body.name)
    .bind(&body.icon)
    .bind(&body.description)
    .bind(&body.category)
    .bind(&body.color)
    .bind(auth.user_id)
    .fetch_one(&mut *tx)
    .await?;

    // Add creator as owner member
    sqlx::query(
        "INSERT INTO server_members (server_id, user_id, role) VALUES ($1, $2, 'owner')"
    )
    .bind(server.id)
    .bind(auth.user_id)
    .execute(&mut *tx)
    .await?;

    // Create default channels
    sqlx::query(
        "INSERT INTO channels (server_id, name, kind, position) VALUES ($1, 'general', 'text', 0), ($1, 'Lounge', 'voice', 0)"
    )
    .bind(server.id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(Json(server))
}

/// Discovery query params
#[derive(Debug, Deserialize)]
pub struct DiscoverQuery {
    pub category: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// GET /api/v1/servers/discover — public servers
pub async fn discover_servers(
    _auth: AuthUser,
    State(state): State<AppState>,
    Query(q): Query<DiscoverQuery>,
) -> AppResult<Json<Vec<Server>>> {
    let limit = q.limit.unwrap_or(20).min(50);
    let offset = q.offset.unwrap_or(0);

    let servers = if let Some(ref cat) = q.category {
        sqlx::query_as::<_, Server>(
            r#"
            SELECT * FROM servers
            WHERE is_public = true AND category = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(cat)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as::<_, Server>(
            r#"
            SELECT * FROM servers
            WHERE is_public = true
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.pool)
        .await?
    };

    Ok(Json(servers))
}

/// GET /api/v1/servers/:id — server details
pub async fn get_server(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(server_id): Path<Uuid>,
) -> AppResult<Json<Server>> {
    // Check membership
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM server_members WHERE server_id = $1 AND user_id = $2)"
    )
    .bind(server_id)
    .bind(auth.user_id)
    .fetch_one(&state.pool)
    .await?;

    if !is_member {
        return Err(AppError::Forbidden("You are not a member of this server".into()));
    }

    let server = sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE id = $1")
        .bind(server_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Server not found".into()))?;

    Ok(Json(server))
}

/// POST /api/v1/servers/:id/join
pub async fn join_server(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(server_id): Path<Uuid>,
) -> AppResult<Json<ServerMember>> {
    // Check server exists and is public
    let server = sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE id = $1")
        .bind(server_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Server not found".into()))?;

    if !server.is_public {
        return Err(AppError::Forbidden("Server is not public".into()));
    }

    let member = sqlx::query_as::<_, ServerMember>(
        r#"
        INSERT INTO server_members (server_id, user_id, role)
        VALUES ($1, $2, 'member')
        ON CONFLICT (server_id, user_id) DO NOTHING
        RETURNING *
        "#,
    )
    .bind(server_id)
    .bind(auth.user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::BadRequest("Already a member".into()))?;

    Ok(Json(member))
}

/// DELETE /api/v1/servers/:id/leave
pub async fn leave_server(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(server_id): Path<Uuid>,
) -> AppResult<axum::http::StatusCode> {
    // Check not owner
    let server = sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE id = $1")
        .bind(server_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Server not found".into()))?;

    if server.owner_id == auth.user_id {
        return Err(AppError::BadRequest("Owner cannot leave the server".into()));
    }

    sqlx::query("DELETE FROM server_members WHERE server_id = $1 AND user_id = $2")
        .bind(server_id)
        .bind(auth.user_id)
        .execute(&state.pool)
        .await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}
