use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::request::Parts,
};
use chrono::Utc;
use domain::repositories::user::UserSessionRepository;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use application::error::AppError;
use crate::state::AppState;

pub struct RequireUserSession {
    pub user_id:    Uuid,
    pub session_id: Uuid,
}

#[async_trait]
impl FromRequestParts<AppState> for RequireUserSession {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let cookie_header = parts.headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let token = cookie_header
            .split(';')
            .map(str::trim)
            .find_map(|part| part.strip_prefix("session="))
            .ok_or_else(|| AppError::Unauthorized("Missing session cookie".into()))?;

        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let token_hash = hex::encode(hasher.finalize());

        let session = state.session_repo
            .find_by_token_hash(&token_hash)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid session".into()))?;

        if Utc::now() > session.expires_at {
            return Err(AppError::Unauthorized("Session expired".into()));
        }

        Ok(RequireUserSession {
            user_id:    session.user_id,
            session_id: session.id,
        })
    }
}
