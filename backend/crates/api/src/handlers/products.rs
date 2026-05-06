use std::sync::Arc;
use axum::{extract::State, Json};
use domain::repositories::product::ProductRepository;
use crate::state::AppState;

/// List active products
#[utoipa::path(get, path = "/products", tag = "products",
    responses((status = 200, description = "OK")))]
pub async fn list_products(
    State(s): State<AppState>,
) -> Result<Json<serde_json::Value>, application::error::AppError> {
    let products = s.product_repo.list(true).await?;
    Ok(Json(serde_json::json!(products)))
}
