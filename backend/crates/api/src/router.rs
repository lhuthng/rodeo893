use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use crate::{
    handlers::{auth, health, membership, occasions, orders, payments, products, staff, stock, user},
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
        // Products (public)
        .route("/api/products",         get(products::list_products))
        .route("/api/products/{slug}",  get(products::get_product))
        // Products (staff)
        .route("/api/staff/products",       post(products::create_product))
        .route("/api/staff/products/{id}",  patch(products::update_product))
        .route("/api/staff/products/{id}",  delete(products::deactivate_product))
        // Stock batches (staff)
        .route("/api/staff/stock-batches",                  get(stock::list_batches))
        .route("/api/staff/stock-batches",                  post(stock::create_batch))
        .route("/api/staff/stock-batches/{id}",             patch(stock::update_batch))
        .route("/api/staff/stock-batches/{id}/release",     post(stock::release_batch))
        // Occasions (public list)
        .route("/api/occasions", get(occasions::list_occasions))
        // Occasions (staff)
        .route("/api/staff/occasions",                  post(occasions::create_occasion))
        .route("/api/staff/occasions/{id}/announce",    post(occasions::announce))
        // Orders
        .route("/api/orders",        post(orders::place_order))
        .route("/api/orders/{id}",   get(orders::get_order))
        .route("/api/orders/{id}",   delete(orders::cancel_order))
        // Payments
        .route("/api/payments/webhook", post(payments::webhook))
        // Staff
        .route("/api/staff/login",                          post(staff::login))
        .route("/api/staff/orders",                         get(staff::list_orders))
        .route("/api/staff/orders/{id}/status",             patch(staff::update_status))
        .route("/api/staff/orders/{id}/tracking",           post(staff::attach_tracking))
        // CORS + tracing
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
