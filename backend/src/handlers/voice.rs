/// Voice/LiveKit REST handlers: token generation, voice state query

use axum::extract::State;
use axum::Json;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::auth::AuthUser;
use crate::error::AppResult;

/// Request body for POST /api/v1/voice/token
#[derive(Debug, Deserialize)]
pub struct VoiceTokenRequest {
    pub channel_id: Uuid,
}

/// Response for POST /api/v1/voice/token
#[derive(Debug, Serialize)]
pub struct VoiceTokenResponse {
    pub token: String,
    pub url: String,
}

/// LiveKit JWT claims
#[derive(Debug, Serialize)]
struct LiveKitClaims {
    exp: usize,
    iss: String,        // API key
    nbf: usize,
    sub: String,        // participant identity
    video: LiveKitVideoGrants,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LiveKitVideoGrants {
    room_join: bool,
    room: String,
    can_publish: bool,
    can_subscribe: bool,
}

/// POST /api/v1/voice/token — generate a LiveKit access token
pub async fn generate_voice_token(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(body): Json<VoiceTokenRequest>,
) -> AppResult<Json<VoiceTokenResponse>> {
    let room_name = format!("channel:{}", body.channel_id);
    let identity = auth.user_id.to_string();

    let now = Utc::now().timestamp() as usize;
    let exp = now + (6 * 60 * 60); // 6 hours

    let claims = LiveKitClaims {
        exp,
        iss: state.config.livekit_api_key.clone(),
        nbf: 0,
        sub: identity,
        video: LiveKitVideoGrants {
            room_join: true,
            room: room_name,
            can_publish: true,
            can_subscribe: true,
        },
    };

    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret(state.config.livekit_api_secret.as_bytes());

    let token = encode(&header, &claims, &key)
        .map_err(|e| crate::error::AppError::Internal(format!("Failed to generate token: {e}")))?;

    Ok(Json(VoiceTokenResponse {
        token,
        url: state.config.livekit_url.clone(),
    }))
}
