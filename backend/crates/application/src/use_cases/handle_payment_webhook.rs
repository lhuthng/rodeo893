use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use domain::{
    entities::payment::{PaymentEvent, PaymentStatus},
    repositories::{
        order::OrderRepository,
        payment::{PaymentEventRepository, PaymentRepository},
    },
};
use crate::error::AppError;

/// Called by the webhook handler after HMAC verification.
pub struct HandlePaymentWebhook<PR, PER, OR> {
    pub payment_repo:       Arc<PR>,
    pub payment_event_repo: Arc<PER>,
    pub order_repo:         Arc<OR>,
}

pub struct WebhookData {
    pub gateway_ref:  String,
    pub new_status:   PaymentStatus,
    pub event_type:   String,
    pub raw_payload:  serde_json::Value,
}

impl<PR, PER, OR> HandlePaymentWebhook<PR, PER, OR>
where
    PR:  PaymentRepository,
    PER: PaymentEventRepository,
    OR:  OrderRepository,
{
    pub async fn execute(&self, data: WebhookData) -> Result<(), AppError> {
        // Always log the event first (immutable)
        let payment = self.payment_repo.find_by_gateway_ref(&data.gateway_ref).await.ok();
        let payment_id = payment.as_ref().map(|p| p.id);

        let event = PaymentEvent {
            id:          Uuid::new_v4(),
            payment_id,
            event_type:  data.event_type,
            raw_payload: data.raw_payload,
            received_at: Utc::now(),
        };
        self.payment_event_repo.append(&event).await?;

        // If we matched a payment, update its status
        if let Some(payment) = payment {
            // Idempotency: don't regress a terminal state
            if matches!(payment.status, PaymentStatus::Completed | PaymentStatus::Refunded) {
                return Ok(());
            }

            self.payment_repo
                .update_status(payment.id, data.new_status.clone(), Some(&data.gateway_ref))
                .await?;

            // If payment completed & this is an order payment, confirm the order
            if data.new_status == PaymentStatus::Completed {
                if let Some(order_id) = payment.order_id {
                    self.order_repo
                        .update_status(order_id, domain::entities::order::OrderStatus::Confirmed)
                        .await?;
                }
            }
        }

        Ok(())
    }
}
