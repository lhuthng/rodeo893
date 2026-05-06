use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use uuid::Uuid;
use application::error::AppError;
use crate::state::AppState;

pub struct RequireStaff {
    pub staff_id: Uuid,
    pub role:     String,
}

#[async_trait]
impl FromRequestParts<AppState> for RequireStaff {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth = parts.headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

        let token = auth.strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid Authorization format".into()))?;

        let claims = state.jwt.decode_access(token)?;

        let staff_id = claims.sub.parse::<Uuid>()
            .map_err(|_| AppError::Unauthorized("Invalid token subject".into()))?;

        Ok(RequireStaff { staff_id, role: claims.role })
    }
}
