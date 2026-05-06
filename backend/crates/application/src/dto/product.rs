use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct CategoryDto {
    pub id:          Uuid,
    pub name:        String,
    pub slug:        String,
    pub description: String,
    pub sort_order:  i16,
}

#[derive(Debug, Serialize)]
pub struct ProductDto {
    pub id:            Uuid,
    pub category_id:   Uuid,
    pub category_slug: String,
    pub name:          String,
    pub slug:          String,
    pub description:   String,
    pub image_url:     Option<String>,
    pub base_price:    Decimal,
    pub currency:      String,
    pub ordering_days: Vec<i16>,
    pub is_active:     bool,
}

#[derive(Debug, Serialize)]
pub struct ProductWithStockDto {
    #[serde(flatten)]
    pub product:        ProductDto,
    pub current_batch:  Option<BatchAvailabilityDto>,
}

#[derive(Debug, Serialize)]
pub struct BatchAvailabilityDto {
    pub batch_id:      Uuid,
    pub week_start:    NaiveDate,
    pub available_qty: i32,
    pub is_released:   bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductInput {
    pub category_id: Uuid,

    #[validate(length(min = 1, max = 200))]
    pub name: String,

    #[validate(length(min = 1, max = 200))]
    pub slug: String,

    pub description: Option<String>,
    pub image_url:   Option<String>,

    #[validate(range(min = 0.0))]
    pub base_price: Decimal,

    /// 0 = Monday … 6 = Sunday
    pub ordering_days: Vec<i16>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProductInput {
    pub name:          Option<String>,
    pub description:   Option<String>,
    pub image_url:     Option<String>,
    pub base_price:    Option<Decimal>,
    pub ordering_days: Option<Vec<i16>>,
    pub is_active:     Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStockBatchInput {
    pub product_id: Uuid,
    pub week_start: NaiveDate,

    #[validate(range(min = 1))]
    pub total_qty: i32,
}

#[derive(Debug, Serialize)]
pub struct StockBatchDto {
    pub id:           Uuid,
    pub product_id:   Uuid,
    pub week_start:   NaiveDate,
    pub total_qty:    i32,
    pub reserved_qty: i32,
    pub available_qty: i32,
    pub is_released:  bool,
    pub created_at:   DateTime<Utc>,
}
