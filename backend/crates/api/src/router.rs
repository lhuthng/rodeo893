use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use crate::{
    handlers::{auth, health, membership, orders, payments, products, staff, user},
    openapi::docs_router,
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Health
        .route("/health", get(health::health))
        // OpenAPI docs at /docs
        .merge(docs_router())
        // Auth
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login",    post(auth::login))
        .route("/api/auth/logout",   post(auth::logout))
        // User
        .route("/api/user/profile",   get(user::get_profile))
        .route("/api/user/addresses", post(user::upsert_address))
        // Membership
        .route("/api/membership",          get(membership::get_status))
        .route("/api/membership/activate", post(membership::activate))
        // Products
        .route("/api/products", get(products::list_products))
        // Orders
        .route("/api/orders",         post(orders::place_order))
        .route("/api/orders/:ref",    get(orders::get_order))
        .route("/api/orders/:id",     delete(orders::cancel_order))
        // Payments
        .route("/api/payments/webhook", post(payments::webhook))
        // Staff
        .route("/api/staff/login",                      post(staff::login))
        .route("/api/staff/orders",                     get(staff::list_orders))
        .route("/api/staff/orders/:id/status",          patch(staff::update_status))
        .route("/api/staff/orders/:id/tracking",        post(staff::attach_tracking))
        // CORS + tracing
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
