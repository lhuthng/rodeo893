use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use application::use_cases::{
    announce_occasion::AnnounceOccasion,
    create_occasion::{CreateOccasion, CreateOccasionInput},
};
use domain::repositories::occasion::OccasionRepository;
use crate::{error::ApiResult, middleware::auth_staff::RequireStaff, state::AppState};

#[derive(Deserialize)]
pub struct CreateOccasionBody {
    pub title:             String,
    pub description:       String,
    pub announcement_date: chrono::DateTime<chrono::Utc>,
    pub order_open_at:     chrono::DateTime<chrono::Utc>,
    pub order_close_at:    chrono::DateTime<chrono::Utc>,
}

/// Create an occasion (staff)
#[utoipa::path(post, path = "/staff/occasions", tag = "occasions",
    responses((status = 201, description = "Created")))]
pub async fn create_occasion(
    State(s): State<AppState>,
    _staff: RequireStaff,
    Json(body): Json<CreateOccasionBody>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let uc = CreateOccasion { occasion_repo: Arc::clone(&s.occasion_repo) };
    let result = uc.execute(CreateOccasionInput {
        title:             body.title,
        description:       body.description,
        announcement_date: body.announcement_date,
        order_open_at:     body.order_open_at,
        order_close_at:    body.order_close_at,
    }).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!(result))))
}

/// List active occasions (public)
#[utoipa::path(get, path = "/occasions", tag = "occasions",
    responses((status = 200, description = "OK")))]
pub async fn list_occasions(
    State(s): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let occasions = s.occasion_repo.list_active().await?;
    Ok(Json(serde_json::json!(occasions)))
}

/// Trigger announcement email blast for an occasion (staff)
#[utoipa::path(post, path = "/staff/occasions/{id}/announce", tag = "occasions",
    responses((status = 200, description = "Announcement queued")))]
pub async fn announce(
    State(s): State<AppState>,
    _staff: RequireStaff,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let uc = AnnounceOccasion {
        occasion_repo:     Arc::clone(&s.occasion_repo),
        announcement_repo: Arc::clone(&s.occ_announce_repo),
        notification_repo: Arc::clone(&s.notification_repo),
        user_repo:         Arc::clone(&s.user_repo),
    };
    let count = uc.execute(id).await?;
    Ok(Json(serde_json::json!({ "emails_queued": count })))
}
