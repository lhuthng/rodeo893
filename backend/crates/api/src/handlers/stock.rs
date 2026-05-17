use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;
use application::{
    dto::product::CreateStockBatchInput,
    use_cases::{
        create_stock_batch::CreateStockBatch,
        release_stock_batch::ReleaseStockBatch,
        update_stock_batch::UpdateStockBatch,
    },
};
use crate::{error::ApiResult, extractors::ValidatedJson, middleware::auth_staff::RequireStaff, state::AppState};

/// Create a stock batch (staff)
#[utoipa::path(post, path = "/staff/stock-batches", tag = "stock",
    responses((status = 201, description = "Created")))]
pub async fn create_batch(
    State(s): State<AppState>,
    _staff: RequireStaff,
    ValidatedJson(input): ValidatedJson<CreateStockBatchInput>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let uc = CreateStockBatch { batch_repo: Arc::clone(&s.stock_repo) };
    Ok((StatusCode::CREATED, Json(serde_json::json!(uc.execute(input).await?))))
}

#[derive(Deserialize)]
pub struct UpdateBatchBody {
    pub total_qty: i32,
}

/// Update stock batch total_qty (staff)
#[utoipa::path(patch, path = "/staff/stock-batches/{id}", tag = "stock",
    responses((status = 200, description = "Updated")))]
pub async fn update_batch(
    State(s): State<AppState>,
    _staff: RequireStaff,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateBatchBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let uc = UpdateStockBatch { batch_repo: Arc::clone(&s.stock_repo) };
    Ok(Json(serde_json::json!(uc.execute(id, body.total_qty).await?)))
}

/// Release a stock batch (staff) — makes it available for ordering
#[utoipa::path(post, path = "/staff/stock-batches/{id}/release", tag = "stock",
    responses((status = 200, description = "Released")))]
pub async fn release_batch(
    State(s): State<AppState>,
    _staff: RequireStaff,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let uc = ReleaseStockBatch { batch_repo: Arc::clone(&s.stock_repo) };
    Ok(Json(serde_json::json!(uc.execute(id).await?)))
}

#[derive(Deserialize)]
pub struct WeekQuery {
    pub week_start: NaiveDate,
}

/// List stock batches for a given week (staff)
#[utoipa::path(get, path = "/staff/stock-batches", tag = "stock",
    responses((status = 200, description = "OK")))]
pub async fn list_batches(
    State(s): State<AppState>,
    _staff: RequireStaff,
    axum::extract::Query(q): axum::extract::Query<WeekQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    use domain::repositories::stock::StockBatchRepository;
    let batches = s.stock_repo.list_by_week(q.week_start).await?;
    Ok(Json(serde_json::json!(batches)))
}
