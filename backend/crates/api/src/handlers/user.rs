use std::sync::Arc;
use axum::{extract::State, Json};
use application::{
    dto::user::{UpsertAddressInput, UserProfileDto},
    use_cases::{get_user_profile::GetUserProfile, upsert_address::UpsertAddress},
};
use crate::{extractors::ValidatedJson, middleware::auth_user::RequireUserSession, state::AppState};

/// Get current user profile
#[utoipa::path(get, path = "/user/profile", tag = "users",
    responses((status = 200, description = "OK")))]
pub async fn get_profile(
    State(s): State<AppState>,
    session: RequireUserSession,
) -> Result<Json<UserProfileDto>, application::error::AppError> {
    let uc = GetUserProfile {
        user_repo:       Arc::clone(&s.user_repo),
        address_repo:    Arc::clone(&s.address_repo),
        membership_repo: Arc::clone(&s.membership_repo),
        credit_repo:     Arc::clone(&s.credit_repo),
    };
    Ok(Json(uc.execute(session.user_id).await?))
}

/// Add or update address
#[utoipa::path(post, path = "/user/addresses", tag = "users",
    request_body = UpsertAddressInput,
    responses((status = 200, description = "OK")))]
pub async fn upsert_address(
    State(s): State<AppState>,
    session: RequireUserSession,
    ValidatedJson(input): ValidatedJson<UpsertAddressInput>,
) -> Result<Json<serde_json::Value>, application::error::AppError> {
    let uc = UpsertAddress {
        user_repo:    Arc::clone(&s.user_repo),
        address_repo: Arc::clone(&s.address_repo),
    };
    let addr = match input.id {
        Some(id) => uc.update(session.user_id, id, input).await?,
        None     => uc.add(session.user_id, input).await?,
    };
    Ok(Json(serde_json::json!(addr)))
}
