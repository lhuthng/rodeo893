use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

impl std::fmt::Display for RefundStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pending   => "pending",
            Self::Approved  => "approved",
            Self::Rejected  => "rejected",
            Self::Completed => "completed",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub id:         Uuid,
    pub payment_id: Uuid,
    pub order_id:   Uuid,
    pub amount:     Decimal,
    pub reason:     String,
    pub status:     RefundStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
