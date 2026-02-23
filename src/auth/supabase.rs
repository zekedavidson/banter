/// Supabase JWT verification middleware.
///
/// Extracts the `Authorization: Bearer <token>` header, decodes the Supabase
/// JWT using the project's JWT secret, and provides the authenticated user's
/// UUID as an `AuthUser` extractor.

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use crate::error::AppError;

/// Authenticated user extracted from Supabase JWT.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

/// Supabase JWT claims (subset we care about).
#[derive(Debug, Deserialize)]
struct SupabaseClaims {
    sub: String,       // user UUID as string
    role: String,      // "authenticated" | "anon" | "service_role"
    #[allow(dead_code)]
    exp: usize,
    #[allow(dead_code)]
    iss: String,       // "supabase"
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // Extract Bearer token from Authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid Authorization format".into()))?;

        // Decode and verify the Supabase JWT
        let key = DecodingKey::from_secret(state.config.supabase_jwt_secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&["supabase"]);

        let token_data = decode::<SupabaseClaims>(token, &key, &validation)
            .map_err(|e| AppError::Unauthorized(format!("Invalid token: {e}")))?;

        if token_data.claims.role != "authenticated" {
            return Err(AppError::Unauthorized("User is not authenticated".into()));
        }

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".into()))?;

        Ok(AuthUser { user_id })
    }
}
