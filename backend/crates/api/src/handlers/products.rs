use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use rust_decimal::Decimal;
use std::collections::HashMap;
use uuid::Uuid;
use sqlx::types::Json as SqlxJson;
use application::{
    dto::product::{CreateProductInput, LocalizedProductDto, UpdateProductInput},
    error::AppError,
    use_cases::{
        create_product::CreateProduct,
        deactivate_product::DeactivateProduct,
        update_product::UpdateProduct,
    },
};
use crate::{error::{ApiError, ApiResult}, extractors::ValidatedJson, middleware::auth_staff::RequireStaff, state::AppState};

// ── Internal row types for the localized JOIN queries ────────────────────────

#[derive(sqlx::FromRow)]
struct LocalizedProductRow {
    pub id:           Uuid,
    pub slug:         String,
    pub localized_slug: String,
    pub frontend_key: Option<String>,
    pub category_slug: String,
    pub category_localized_slug: String,
    pub category_name: String,
    pub name:         String,
    pub description:  String,
    pub image_url:    Option<String>,
    pub image_alt:    Option<String>,
    pub base_price:   Decimal,
    pub currency:     String,
    pub plating:      Option<String>,
    pub notes:        SqlxJson<Vec<String>>,
}

#[derive(sqlx::FromRow)]
struct ResolvedCategoryRow {
    pub category_slug: String,
}

#[derive(sqlx::FromRow)]
struct ResolvedProductRow {
    pub product_slug:  String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct ResolvedCatalogPathDto {
    pub category_slug: String,
    pub category_localized_slug: String,
    pub product_slug: Option<String>,
    pub product_localized_slug: Option<String>,
}

async fn query_localized(pool: &sqlx::PgPool, lang: &str) -> Result<Vec<LocalizedProductDto>, sqlx::Error> {
    let rows = sqlx::query_as::<_, LocalizedProductRow>(r#"
        SELECT
            p.id,
            p.slug,
            COALESCE(pt.slug, p.slug) AS localized_slug,
            p.frontend_key,
            pc.slug        AS category_slug,
            COALESCE(pct.slug, pc.slug) AS category_localized_slug,
            COALESCE(pct.name, initcap(replace(pc.slug, '-', ' '))) AS category_name,
            COALESCE(pt.name,        p.name)        AS name,
            COALESCE(pt.description, p.description) AS description,
            p.image_url,
            p.image_alt,
            p.base_price,
            p.currency,
            p.plating,
            COALESCE(pt.notes, '[]'::jsonb) AS notes
        FROM products p
        JOIN product_categories pc ON pc.id = p.category_id
        LEFT JOIN product_translations pt
            ON pt.product_id = p.id AND pt.lang = $1
        LEFT JOIN product_category_translations pct
            ON pct.category_id = pc.id AND pct.lang = $1
        WHERE p.is_active = true
        ORDER BY pc.sort_order, p.name
    "#)
    .bind(lang)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| {
        LocalizedProductDto {
            id:           r.id,
            slug:         r.slug,
            localized_slug: r.localized_slug,
            frontend_key: r.frontend_key,
            category_slug: r.category_slug,
            category_localized_slug: r.category_localized_slug,
            category_name: r.category_name,
            name:         r.name,
            description:  r.description,
            image_url:    r.image_url,
            image_alt:    r.image_alt,
            base_price:   r.base_price,
            currency:     r.currency,
            notes:        r.notes.0,
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

/// Resolve category/product slugs from any supported language to canonical and requested-language slugs.
#[utoipa::path(get, path = "/products/resolve-path", tag = "products",
    params(
        ("category" = String, Query, description = "Category slug in any supported language"),
        ("product" = Option<String>, Query, description = "Product slug in any supported language"),
        ("lang" = Option<String>, Query, description = "Target language for localized output slugs"),
    ),
    responses(
        (status = 200, description = "Resolved", body = ResolvedCatalogPathDto),
        (status = 404, description = "Not found")
    ))]
pub async fn resolve_catalog_path(
    State(s): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> ApiResult<Json<ResolvedCatalogPathDto>> {
    let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("en");
    let incoming_category = params
        .get("category")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ApiError(AppError::Validation("Missing query parameter 'category'".to_string())))?;
    let incoming_product = params
        .get("product")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);

    let resolved_category = sqlx::query_as::<_, ResolvedCategoryRow>(
        r#"
        SELECT pc.slug AS category_slug
        FROM product_categories pc
        LEFT JOIN product_category_translations pct ON pct.category_id = pc.id
        WHERE pc.slug = $1 OR pct.slug = $1
        LIMIT 1
        "#,
    )
    .bind(incoming_category)
    .fetch_optional(&s.db)
    .await
    .map_err(|e| ApiError(AppError::Internal(e.to_string())))?
    .ok_or_else(|| {
        ApiError(AppError::NotFound(format!(
            "Category '{incoming_category}' not found"
        )))
    })?;

    let category_localized_slug = sqlx::query_scalar::<_, String>(
        r#"
        SELECT COALESCE(pct.slug, pc.slug) AS localized_slug
        FROM product_categories pc
        LEFT JOIN product_category_translations pct
            ON pct.category_id = pc.id AND pct.lang = $2
        WHERE pc.slug = $1
        LIMIT 1
        "#,
    )
    .bind(&resolved_category.category_slug)
    .bind(lang)
    .fetch_one(&s.db)
    .await
    .map_err(|e| ApiError(AppError::Internal(e.to_string())))?;

    let (product_slug, product_localized_slug) = match incoming_product {
        None => (None, None),
        Some(incoming_product_slug) => {
            let resolved_product = sqlx::query_as::<_, ResolvedProductRow>(
                r#"
                SELECT
                    p.slug  AS product_slug
                FROM products p
                JOIN product_categories pc ON pc.id = p.category_id
                LEFT JOIN product_category_translations pct ON pct.category_id = pc.id
                LEFT JOIN product_translations pt ON pt.product_id = p.id
                WHERE p.is_active = true
                  AND (pc.slug = $1 OR pct.slug = $1)
                  AND (p.slug = $2 OR pt.slug = $2)
                LIMIT 1
                "#,
            )
            .bind(&resolved_category.category_slug)
            .bind(&incoming_product_slug)
            .fetch_optional(&s.db)
            .await
            .map_err(|e| ApiError(AppError::Internal(e.to_string())))?
            .ok_or_else(|| {
                ApiError(AppError::NotFound(format!(
                    "Product '{incoming_product_slug}' not found"
                )))
            })?;

            let localized_product_slug = sqlx::query_scalar::<_, String>(
                r#"
                SELECT COALESCE(pt.slug, p.slug) AS localized_slug
                FROM products p
                LEFT JOIN product_translations pt
                    ON pt.product_id = p.id AND pt.lang = $2
                WHERE p.slug = $1
                LIMIT 1
                "#,
            )
            .bind(&resolved_product.product_slug)
            .bind(lang)
            .fetch_one(&s.db)
            .await
            .map_err(|e| ApiError(AppError::Internal(e.to_string())))?;

            (
                Some(resolved_product.product_slug),
                Some(localized_product_slug),
            )
        }
    };

    Ok(Json(ResolvedCatalogPathDto {
        category_slug: resolved_category.category_slug,
        category_localized_slug,
        product_slug,
        product_localized_slug,
    }))
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
