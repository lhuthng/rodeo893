use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCategory {
    pub id:          Uuid,
    pub name:        String,
    pub slug:        String,
    pub description: String,
    pub sort_order:  i16,
    pub created_at:  DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id:          Uuid,
    pub category_id: Uuid,
    pub name:        String,
    pub slug:        String,
    pub description: String,
    pub image_url:   Option<String>,
    pub base_price:  Decimal,
    pub currency:    String,
    pub is_active:   bool,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
}

/// Day of week: 0 = Monday, 6 = Sunday
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductOrderingDay {
    pub product_id:   Uuid,
    pub day_of_week:  i16,
}
