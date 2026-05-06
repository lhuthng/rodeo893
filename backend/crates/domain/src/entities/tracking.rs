use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTracking {
    pub id:              Uuid,
    pub order_id:        Uuid,
    pub carrier_code:    String,
    pub tracking_number: String,
    pub tracking_url:    Option<String>,
    pub last_status:     Option<String>,
    pub last_checked_at: Option<DateTime<Utc>>,
    pub raw_status:      Value,
    pub created_at:      DateTime<Utc>,
    pub updated_at:      DateTime<Utc>,
}
