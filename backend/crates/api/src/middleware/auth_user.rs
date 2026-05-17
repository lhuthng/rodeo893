use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
};
use chrono::Utc;
use domain::repositories::user::UserSessionRepository;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use application::error::AppError;
use crate::{error::ApiError, state::AppState};

pub struct RequireUserSession {
    pub user_id:    Uuid,
    pub session_id: Uuid,
}

impl FromRequestParts<AppState> for RequireUserSession {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let cookie_header = parts.headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let token = cookie_header
            .split(';')
            .map(str::trim)
            .find_map(|part| part.strip_prefix("session="))
            .ok_or_else(|| ApiError(AppError::Unauthorized("Missing session cookie".into())))?;

        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let token_hash = hex::encode(hasher.finalize());

        let session = state.session_repo
            .find_by_token_hash(&token_hash)
            .await
            .map_err(|_| ApiError(AppError::Unauthorized("Invalid session".into())))?;

        if Utc::now() > session.expires_at {
            return Err(ApiError(AppError::Unauthorized("Session expired".into())));
        }

        Ok(RequireUserSession {
            user_id:    session.user_id,
            session_id: session.id,
        })
    }
}

impl OptionalFromRequestParts<AppState> for RequireUserSession {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Option<Self>, Self::Rejection> {
        let cookie_header = parts.headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let token = match cookie_header
            .split(';')
            .map(str::trim)
            .find_map(|part| part.strip_prefix("session="))
        {
            Some(t) => t.to_owned(),
            None => return Ok(None),
        };

        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let token_hash = hex::encode(hasher.finalize());

        let session = match state.session_repo.find_by_token_hash(&token_hash).await {
            Ok(s) => s,
            Err(_) => return Ok(None),
        };

        if Utc::now() > session.expires_at {
            return Ok(None);
        }

        Ok(Some(RequireUserSession {
            user_id:    session.user_id,
            session_id: session.id,
        }))
    }
}
