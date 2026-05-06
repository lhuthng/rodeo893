use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::state::AppState;

/// Health check
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses((status = 200, description = "OK"))
)]
pub async fn health(State(_): State<AppState>) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
