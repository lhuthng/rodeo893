use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use application::{
    dto::product::{CreateProductInput, UpdateProductInput},
    use_cases::{
        create_product::CreateProduct,
        deactivate_product::DeactivateProduct,
        update_product::UpdateProduct,
    },
};
use domain::repositories::product::ProductRepository;
use crate::{error::ApiResult, extractors::ValidatedJson, middleware::auth_staff::RequireStaff, state::AppState};

/// List active products
#[utoipa::path(get, path = "/products", tag = "products",
    responses((status = 200, description = "OK")))]
pub async fn list_products(
    State(s): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let products = s.product_repo.list_active().await?;
    Ok(Json(serde_json::json!(products)))
}

/// Create a product (staff)
#[utoipa::path(post, path = "/staff/products", tag = "products",
    responses((status = 201, description = "Created")))]
pub async fn create_product(
    State(s): State<AppState>,
    _staff: RequireStaff,
    ValidatedJson(input): ValidatedJson<CreateProductInput>,
) -> ApiResult<(StatusCode, Json<serde_json::Value>)> {
    let uc = CreateProduct {
        product_repo:  Arc::clone(&s.product_repo),
        category_repo: Arc::clone(&s.product_cat_repo),
        ordering_repo: Arc::clone(&s.ordering_day_repo),
    };
    Ok((StatusCode::CREATED, Json(serde_json::json!(uc.execute(input).await?))))
}

/// Update a product (staff)
#[utoipa::path(patch, path = "/staff/products/{id}", tag = "products",
    responses((status = 204, description = "Updated")))]
pub async fn update_product(
    State(s): State<AppState>,
    _staff: RequireStaff,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateProductInput>,
) -> ApiResult<StatusCode> {
    let uc = UpdateProduct {
        product_repo:  Arc::clone(&s.product_repo),
        ordering_repo: Arc::clone(&s.ordering_day_repo),
    };
    uc.execute(id, input).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Deactivate a product (staff)
#[utoipa::path(delete, path = "/staff/products/{id}", tag = "products",
    responses((status = 204, description = "Deactivated")))]
pub async fn deactivate_product(
    State(s): State<AppState>,
    _staff: RequireStaff,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let uc = DeactivateProduct { product_repo: Arc::clone(&s.product_repo) };
    uc.execute(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
