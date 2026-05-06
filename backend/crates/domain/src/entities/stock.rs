use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockBatch {
    pub id:           Uuid,
    pub product_id:   Uuid,
    /// Always a Monday
    pub week_start:   NaiveDate,
    pub total_qty:    i32,
    pub reserved_qty: i32,
    pub is_released:  bool,
    pub created_at:   DateTime<Utc>,
    pub updated_at:   DateTime<Utc>,
}

impl StockBatch {
    pub fn available_qty(&self) -> i32 {
        self.total_qty - self.reserved_qty
    }
}
