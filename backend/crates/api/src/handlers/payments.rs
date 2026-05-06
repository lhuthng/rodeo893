use std::sync::Arc;
use axum::{extract::State, http::StatusCode, Json};
use application::use_cases::handle_payment_webhook::{HandlePaymentWebhook, WebhookData};
use domain::entities::payment::PaymentStatus;
use crate::state::AppState;

/// Receive payment webhook (HMAC verification done in real impl)
#[utoipa::path(post, path = "/payments/webhook", tag = "payments",
    responses((status = 200, description = "OK")))]
pub async fn webhook(
    State(s): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<StatusCode, application::error::AppError> {
    // TODO: verify HMAC per provider before processing
    let gateway_ref = body["gateway_ref"].as_str().unwrap_or("").to_string();
    let status_str  = body["status"].as_str().unwrap_or("");
    let event_type  = body["event_type"].as_str().unwrap_or("payment.update").to_string();

    let new_status = match status_str {
        "completed" => PaymentStatus::Completed,
        "failed"    => PaymentStatus::Failed,
        _           => PaymentStatus::Pending,
    };

    let uc = HandlePaymentWebhook {
        payment_repo:       Arc::clone(&s.payment_repo),
        payment_event_repo: Arc::clone(&s.payment_event_repo),
        order_repo:         Arc::clone(&s.order_repo),
    };

    uc.execute(WebhookData {
        gateway_ref,
        new_status,
        event_type,
        raw_payload: body,
    }).await?;

    Ok(StatusCode::OK)
}
