use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pending    => "pending",
            Self::Processing => "processing",
            Self::Completed  => "completed",
            Self::Failed     => "failed",
            Self::Refunded   => "refunded",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProvider {
    pub id:          Uuid,
    pub name:        String,
    pub code:        String,
    pub market:      String,
    pub is_active:   bool,
    pub config_json: Value,
    pub created_at:  DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id:              Uuid,
    pub order_id:        Option<Uuid>,
    pub provider_id:     Uuid,
    pub amount:          Decimal,
    pub currency:        String,
    pub status:          PaymentStatus,
    pub gateway_ref:     Option<String>,
    pub gateway_payload: Value,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}

/// Immutable webhook event log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentEvent {
    pub id:          Uuid,
    pub payment_id:  Option<Uuid>,
    pub event_type:  String,
    pub raw_payload: Value,
    pub received_at: DateTime<Utc>,
}
