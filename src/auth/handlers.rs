/// Auth-related REST handlers: GET /auth/me, PATCH /auth/me

use axum::extract::State;
use axum::Json;
use serde::Deserialize;

use crate::AppState;
use crate::auth::AuthUser;
use crate::error::{AppError, AppResult};
use crate::models::Profile;

/// GET /api/v1/auth/me — return the authenticated user's profile
pub async fn get_me(
    auth: AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Profile>> {
    let profile = sqlx::query_as::<_, Profile>(
        "SELECT * FROM profiles WHERE id = $1"
    )
    .bind(auth.user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Profile not found".into()))?;

    Ok(Json(profile))
}

/// Request body for PATCH /auth/me
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

/// PATCH /api/v1/auth/me — update the authenticated user's profile
pub async fn update_me(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<UpdateProfileRequest>,
) -> AppResult<Json<Profile>> {
    let profile = sqlx::query_as::<_, Profile>(
        r#"
        UPDATE profiles
        SET
            username     = COALESCE($2, username),
            display_name = COALESCE($3, display_name),
            avatar_url   = COALESCE($4, avatar_url),
            updated_at   = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(auth.user_id)
    .bind(body.username)
    .bind(body.display_name)
    .bind(body.avatar_url)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(profile))
}
