use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use application::{
    dto::order::PlaceOrderInput,
    use_cases::{
        cancel_order::CancelOrder,
        get_order_by_reference::GetOrderByReference,
        place_order::PlaceOrder,
    },
};
use crate::{extractors::ValidatedJson, middleware::auth_user::RequireUserSession, state::AppState};

/// Place a new order
#[utoipa::path(post, path = "/orders", tag = "orders",
    request_body = PlaceOrderInput,
    responses((status = 201, description = "Created")))]
pub async fn place_order(
    State(s): State<AppState>,
    session: Option<RequireUserSession>,
    ValidatedJson(input): ValidatedJson<PlaceOrderInput>,
) -> Result<(StatusCode, Json<serde_json::Value>), application::error::AppError> {
    let uc = PlaceOrder {
        order_repo:       Arc::clone(&s.order_repo),
        item_repo:        Arc::clone(&s.order_item_repo),
        delivery_repo:    Arc::clone(&s.delivery_repo),
        batch_repo:       Arc::clone(&s.stock_repo),
        occ_product_repo: Arc::clone(&s.occ_product_repo),
        provider_repo:    Arc::clone(&s.payment_provider_repo),
        payment_repo:     Arc::clone(&s.payment_repo),
        payment_gateway:  Arc::clone(&s.payment_gateway),
    };
    let user_id = session.map(|s| s.user_id);
    let resp = uc.execute(user_id, input).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!(resp))))
}

/// Get order by reference code
#[utoipa::path(get, path = "/orders/{reference}", tag = "orders",
    responses((status = 200, description = "OK")))]
pub async fn get_order(
    State(s): State<AppState>,
    Path(reference): Path<String>,
) -> Result<Json<serde_json::Value>, application::error::AppError> {
    let uc = GetOrderByReference {
        order_repo:    Arc::clone(&s.order_repo),
        item_repo:     Arc::clone(&s.order_item_repo),
        delivery_repo: Arc::clone(&s.delivery_repo),
        payment_repo:  Arc::clone(&s.payment_repo),
        tracking_repo: Arc::clone(&s.tracking_repo),
        product_repo:  Arc::clone(&s.product_repo),
    };
    Ok(Json(serde_json::json!(uc.execute(&reference).await?)))
}

/// Cancel an order
#[utoipa::path(delete, path = "/orders/{id}", tag = "orders",
    responses((status = 204, description = "Cancelled")))]
pub async fn cancel_order(
    State(s): State<AppState>,
    session: RequireUserSession,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, application::error::AppError> {
    let uc = CancelOrder {
        order_repo:       Arc::clone(&s.order_repo),
        item_repo:        Arc::clone(&s.order_item_repo),
        batch_repo:       Arc::clone(&s.stock_repo),
        occ_product_repo: Arc::clone(&s.occ_product_repo),
    };
    uc.execute(id, Some(session.user_id)).await?;
    Ok(StatusCode::NO_CONTENT)
}
