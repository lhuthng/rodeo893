use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occasion {
    pub id:                Uuid,
    pub title:             String,
    pub description:       String,
    pub announcement_date: DateTime<Utc>,
    pub order_open_at:     DateTime<Utc>,
    pub order_close_at:    DateTime<Utc>,
    pub is_active:         bool,
    pub created_at:        DateTime<Utc>,
}

impl Occasion {
    pub fn is_open_for_ordering(&self) -> bool {
        let now = Utc::now();
        self.is_active && now >= self.order_open_at && now <= self.order_close_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccasionProduct {
    pub id:           Uuid,
    pub occasion_id:  Uuid,
    pub product_id:   Uuid,
    pub total_qty:    i32,
    pub reserved_qty: i32,
    pub unit_price:   Decimal,
}

impl OccasionProduct {
    pub fn available_qty(&self) -> i32 {
        self.total_qty - self.reserved_qty
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccasionAnnouncement {
    pub id:              Uuid,
    pub occasion_id:     Uuid,
    pub sent_at:         DateTime<Utc>,
    pub recipient_count: i32,
    pub status:          String,
}
