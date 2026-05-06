use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use axum::Router;
use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Bakery API",
        version = "1.0.0",
        description = "Bakery pre-order system with membership, cashback, and delivery tracking"
    ),
    paths(
        crate::handlers::health::health,
        crate::handlers::auth::register,
        crate::handlers::auth::login,
        crate::handlers::auth::logout,
        crate::handlers::user::get_profile,
        crate::handlers::user::upsert_address,
        crate::handlers::membership::get_status,
        crate::handlers::membership::activate,
        crate::handlers::products::list_products,
        crate::handlers::orders::place_order,
        crate::handlers::orders::get_order,
        crate::handlers::orders::cancel_order,
        crate::handlers::payments::webhook,
        crate::handlers::staff::login as staff_login,
        crate::handlers::staff::list_orders,
        crate::handlers::staff::update_status,
        crate::handlers::staff::attach_tracking,
    ),
    tags(
        (name = "health",     description = "Health check"),
        (name = "auth",       description = "User authentication"),
        (name = "users",      description = "User profile & addresses"),
        (name = "membership", description = "Membership & credit"),
        (name = "products",   description = "Product catalog"),
        (name = "orders",     description = "Order lifecycle"),
        (name = "payments",   description = "Payment webhooks"),
        (name = "staff",      description = "Staff management endpoints"),
    )
)]
pub struct ApiDoc;

pub fn docs_router() -> Router<AppState> {
    Router::new().merge(Scalar::with_url("/docs", ApiDoc::openapi()))
}
