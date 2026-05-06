use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    PendingPayment,
    Confirmed,
    Processing,
    OutForDelivery,
    Delivered,
    Cancelled,
    Refunded,
}

impl OrderStatus {
    pub fn is_cancellable(&self) -> bool {
        matches!(self, Self::PendingPayment | Self::Confirmed)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PendingPayment  => "pending_payment",
            Self::Confirmed       => "confirmed",
            Self::Processing      => "processing",
            Self::OutForDelivery  => "out_for_delivery",
            Self::Delivered       => "delivered",
            Self::Cancelled       => "cancelled",
            Self::Refunded        => "refunded",
        }
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id:             Uuid,
    pub reference_code: String,
    pub user_id:        Option<Uuid>,
    pub status:         OrderStatus,
    pub total_amount:   Decimal,
    pub currency:       String,
    pub note:           Option<String>,
    pub created_at:     DateTime<Utc>,
    pub updated_at:     DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id:                  Uuid,
    pub order_id:            Uuid,
    pub product_id:          Uuid,
    pub batch_id:            Option<Uuid>,
    pub occasion_product_id: Option<Uuid>,
    pub quantity:            i32,
    pub unit_price:          Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDeliveryInfo {
    pub id:             Uuid,
    pub order_id:       Uuid,
    pub recipient_name: String,
    pub phone:          String,
    pub street:         String,
    pub ward:           String,
    pub district:       String,
    pub city:           String,
    pub country:        String,
    pub delivery_note:  Option<String>,
}
