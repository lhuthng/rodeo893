use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct MembershipDto {
    pub id:           Uuid,
    pub is_active:    bool,
    pub activated_at: DateTime<Utc>,
    pub expires_at:   Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CreditBalanceDto {
    pub balance:  Decimal,
    pub currency: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ActivateMembershipInput {
    /// Payment provider code to use for the top-up
    #[validate(length(min = 1))]
    pub provider_code: String,
}

#[derive(Debug, Serialize)]
pub struct ActivateMembershipResponse {
    pub payment_id:       Uuid,
    pub amount:           Decimal,
    pub currency:         String,
    pub instructions:     serde_json::Value,
}
