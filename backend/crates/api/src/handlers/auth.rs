use std::sync::Arc;
use axum::{extract::State, http::{HeaderMap, StatusCode}, Json};
use application::{
    dto::auth::{AuthResponse, LoginInput, RegisterUserInput},
    use_cases::{login_user::LoginUser, logout_user::LogoutUser, register_user::RegisterUser},
};
use crate::{
    extractors::ValidatedJson,
    middleware::auth_user::RequireUserSession,
    state::AppState,
};

/// Register a new user
#[utoipa::path(post, path = "/auth/register", tag = "auth",
    request_body = RegisterUserInput,
    responses((status = 201, description = "Created")))]
pub async fn register(
    State(s): State<AppState>,
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<RegisterUserInput>,
) -> Result<(StatusCode, Json<AuthResponse>), application::error::AppError> {
    let uc = RegisterUser {
        user_repo:    Arc::clone(&s.user_repo),
        session_repo: Arc::clone(&s.session_repo),
        hasher:       Arc::clone(&s.hasher),
    };
    let ip = headers.get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let ua = headers.get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let resp = uc.execute(input, ip, ua).await?;
    Ok((StatusCode::CREATED, Json(resp)))
}

/// Login
#[utoipa::path(post, path = "/auth/login", tag = "auth",
    request_body = LoginInput,
    responses((status = 200, description = "OK")))]
pub async fn login(
    State(s): State<AppState>,
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<LoginInput>,
) -> Result<Json<AuthResponse>, application::error::AppError> {
    let uc = LoginUser {
        user_repo:    Arc::clone(&s.user_repo),
        session_repo: Arc::clone(&s.session_repo),
        hasher:       Arc::clone(&s.hasher),
    };
    let ip = headers.get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let ua = headers.get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    Ok(Json(uc.execute(input, ip, ua).await?))
}

/// Logout
#[utoipa::path(post, path = "/auth/logout", tag = "auth",
    responses((status = 204, description = "No content")))]
pub async fn logout(
    State(s): State<AppState>,
    session: RequireUserSession,
) -> Result<StatusCode, application::error::AppError> {
    let uc = LogoutUser { session_repo: Arc::clone(&s.session_repo) };
    uc.execute(session.session_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
