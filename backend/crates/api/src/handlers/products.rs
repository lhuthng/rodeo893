use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;
use application::{
    dto::product::{CreateProductInput, LocalizedProductDto, UpdateProductInput},
    error::AppError,
    use_cases::{
        create_product::CreateProduct,
        deactivate_product::DeactivateProduct,
        update_product::UpdateProduct,
    },
};
use domain::repositories::product::ProductRepository;
use crate::{error::{ApiError, ApiResult}, extractors::ValidatedJson, middleware::auth_staff::RequireStaff, state::AppState};

// ── Internal row types for the localized JOIN queries ────────────────────────

#[derive(sqlx::FromRow)]
struct LocalizedProductRow {
    pub id:           Uuid,
    pub slug:         String,
    pub frontend_key: Option<String>,
    pub category_slug: String,
    pub name:         String,
    pub description:  String,
    pub image_url:    Option<String>,
    pub image_alt:    Option<String>,
    pub base_price:   Decimal,
    pub currency:     String,
    pub plating:      Option<String>,
}

#[derive(sqlx::FromRow)]
struct NoteRow {
    pub product_id: Uuid,
    pub body:       String,
}

async fn query_localized(pool: &sqlx::PgPool, lang: &str) -> Result<Vec<LocalizedProductDto>, sqlx::Error> {
    let rows = sqlx::query_as::<_, LocalizedProductRow>(r#"
        SELECT
            p.id,
            p.slug,
            p.frontend_key,
            pc.slug        AS category_slug,
            COALESCE(pt.name,        p.name)        AS name,
            COALESCE(pt.description, p.description) AS description,
            p.image_url,
            p.image_alt,
            p.base_price,
            p.currency,
            p.plating
        FROM products p
        JOIN product_categories pc ON pc.id = p.category_id
        LEFT JOIN product_translations pt
            ON pt.product_id = p.id AND pt.lang = $1
        WHERE p.is_active = true
        ORDER BY pc.sort_order, p.name
    "#)
    .bind(lang)
    .fetch_all(pool)
    .await?;

    if rows.is_empty() {
        return Ok(vec![]);
    }

    let ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
    let notes = sqlx::query_as::<_, NoteRow>(r#"
        SELECT product_id, body
        FROM product_notes
        WHERE product_id = ANY($1) AND lang = $2
        ORDER BY product_id, sort_order
    "#)
    .bind(&ids)
    .bind(lang)
    .fetch_all(pool)
    .await?;

    let mut notes_map: HashMap<Uuid, Vec<String>> = HashMap::new();
    for n in notes {
        notes_map.entry(n.product_id).or_default().push(n.body);
    }

    Ok(rows.into_iter().map(|r| {
        let notes = notes_map.remove(&r.id).unwrap_or_default();
        LocalizedProductDto {
            id:           r.id,
            slug:         r.slug,
            frontend_key: r.frontend_key,
            category_slug: r.category_slug,
            name:         r.name,
            description:  r.description,
            image_url:    r.image_url,
            image_alt:    r.image_alt,
            base_price:   r.base_price,
            currency:     r.currency,
            notes,
            plating:      r.plating,
        }
    }).collect())
}

// ── Public handlers ──────────────────────────────────────────────────────────

/// List active products with optional localisation (`?lang=en`)
#[utoipa::path(get, path = "/products", tag = "products",
    params(("lang" = Option<String>, Query, description = "Language code (en|vi)")),
    responses((status = 200, description = "OK")))]
pub async fn list_products(
    State(s): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> ApiResult<Json<Vec<LocalizedProductDto>>> {
    let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("en");
    let products = query_localized(&s.db, lang).await
        .map_err(|e| ApiError(AppError::Internal(e.to_string())))?;
    Ok(Json(products))
}

/// Get a single product
#[utoipa::path(get, path = "/products/{slug}", tag = "products",
    params(
        ("slug" = String, Path, description = "Product URL slug"),
        ("lang" = Option<String>, Query, description = "Language code (en|vi)"),
    ),
    responses((status = 200, description = "OK"), (status = 404, description = "Not found")))]
pub async fn get_product(
    State(s): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> ApiResult<Json<LocalizedProductDto>> {
    let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("en");
    let products = query_localized(&s.db, lang).await
        .map_err(|e| ApiError(AppError::Internal(e.to_string())))?;
    let product = products.into_iter().find(|p| p.slug == slug)
        .ok_or_else(|| ApiError(AppError::NotFound(format!("Product '{slug}' not found"))))?;
    Ok(Json(product))
}

// ── Staff handlers ───────────────────────────────────────────────────────────

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
