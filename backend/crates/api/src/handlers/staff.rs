use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use application::{
    dto::staff::{AttachTrackingInput, StaffLoginInput, UpdateOrderStatusInput},
    use_cases::{
        attach_tracking::AttachTracking,
        list_orders::ListOrders,
        staff_login::StaffLogin,
        staff_refresh::StaffRefresh,
        update_order_status::UpdateOrderStatus,
    },
};
use crate::{extractors::ValidatedJson, middleware::auth_staff::RequireStaff, state::AppState};

/// Staff login
#[utoipa::path(post, path = "/staff/login", tag = "staff",
    request_body = StaffLoginInput,
    responses((status = 200, description = "OK")))]
pub async fn login(
    State(s): State<AppState>,
    ValidatedJson(input): ValidatedJson<StaffLoginInput>,
) -> Result<Json<serde_json::Value>, application::error::AppError> {
    let uc = StaffLogin {
        staff_repo:   Arc::clone(&s.staff_repo),
        refresh_repo: Arc::clone(&s.refresh_token_repo),
        hasher:       Arc::clone(&s.hasher),
        jwt:          Arc::clone(&s.jwt),
    };
    Ok(Json(serde_json::json!(uc.execute(input).await?)))
}

#[derive(Deserialize)]
pub struct OrderListQuery {
    pub status: Option<String>,
    pub limit:  Option<i64>,
    pub offset: Option<i64>,
}

/// List all orders (staff)
#[utoipa::path(get, path = "/staff/orders", tag = "staff",
    responses((status = 200, description = "OK")))]
pub async fn list_orders(
    State(s): State<AppState>,
    _staff: RequireStaff,
    Query(q): Query<OrderListQuery>,
) -> Result<Json<serde_json::Value>, application::error::AppError> {
    let uc = ListOrders { order_repo: Arc::clone(&s.order_repo) };
    let (orders, total) = uc.execute(q.status, None, q.limit.unwrap_or(50), q.offset.unwrap_or(0)).await?;
    Ok(Json(serde_json::json!({ "data": orders, "total": total })))
}

/// Update order status (staff)
#[utoipa::path(patch, path = "/staff/orders/{id}/status", tag = "staff",
    request_body = UpdateOrderStatusInput,
    responses((status = 200, description = "OK")))]
pub async fn update_status(
    State(s): State<AppState>,
    _staff: RequireStaff,
    Path(id): Path<uuid::Uuid>,
    Json(input): Json<UpdateOrderStatusInput>,
) -> Result<StatusCode, application::error::AppError> {
    let uc = UpdateOrderStatus { order_repo: Arc::clone(&s.order_repo) };
    uc.execute(id, &input.status).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Attach tracking info to order (staff)
#[utoipa::path(post, path = "/staff/orders/{id}/tracking", tag = "staff",
    request_body = AttachTrackingInput,
    responses((status = 201, description = "Created")))]
pub async fn attach_tracking(
    State(s): State<AppState>,
    _staff: RequireStaff,
    Path(id): Path<uuid::Uuid>,
    ValidatedJson(input): ValidatedJson<AttachTrackingInput>,
) -> Result<StatusCode, application::error::AppError> {
    let uc = AttachTracking { tracking_repo: Arc::clone(&s.tracking_repo) };
    uc.execute(id, input).await?;
    Ok(StatusCode::CREATED)
}
