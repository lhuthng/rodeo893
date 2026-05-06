use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use rust_decimal::Decimal;

#[derive(Debug, Serialize)]
pub struct PaymentProviderDto {
    pub id:     Uuid,
    pub name:   String,
    pub code:   String,
    pub market: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RequestRefundInput {
    pub order_id: Uuid,
    #[validate(length(min = 1))]
    pub reason:   String,
}

#[derive(Debug, Serialize)]
pub struct RefundDto {
    pub id:         Uuid,
    pub order_id:   Uuid,
    pub amount:     Decimal,
    pub reason:     String,
    pub status:     String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
