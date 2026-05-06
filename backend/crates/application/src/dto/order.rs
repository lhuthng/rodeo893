use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct PlaceOrderInput {
    pub items: Vec<OrderItemInput>,

    pub delivery: DeliveryInfoInput,

    /// Optional note from customer
    pub note: Option<String>,

    /// Payment provider code
    #[validate(length(min = 1))]
    pub provider_code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct OrderItemInput {
    pub product_id: Uuid,
    pub source:     OrderItemSource,

    #[validate(range(min = 1))]
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrderItemSource {
    WeeklyBatch { batch_id: Uuid },
    OccasionProduct { occasion_product_id: Uuid },
}

#[derive(Debug, Deserialize, Validate)]
pub struct DeliveryInfoInput {
    #[validate(length(min = 1, max = 100))]
    pub recipient_name: String,

    #[validate(length(min = 9, max = 15))]
    pub phone: String,

    #[validate(length(min = 1))]
    pub street: String,

    #[validate(length(min = 1))]
    pub ward: String,

    #[validate(length(min = 1))]
    pub district: String,

    #[validate(length(min = 1))]
    pub city: String,

    pub delivery_note: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderSummaryDto {
    pub id:             Uuid,
    pub reference_code: String,
    pub status:         String,
    pub total_amount:   Decimal,
    pub currency:       String,
    pub item_count:     i32,
    pub created_at:     DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct OrderDetailDto {
    pub id:             Uuid,
    pub reference_code: String,
    pub status:         String,
    pub total_amount:   Decimal,
    pub currency:       String,
    pub note:           Option<String>,
    pub items:          Vec<OrderItemDto>,
    pub delivery:       DeliveryInfoDto,
    pub payment:        Option<PaymentSummaryDto>,
    pub tracking:       Option<TrackingDto>,
    pub created_at:     DateTime<Utc>,
    pub updated_at:     DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemDto {
    pub product_id:   Uuid,
    pub product_name: String,
    pub quantity:     i32,
    pub unit_price:   Decimal,
    pub subtotal:     Decimal,
}

#[derive(Debug, Serialize)]
pub struct DeliveryInfoDto {
    pub recipient_name: String,
    pub phone:          String,
    pub street:         String,
    pub ward:           String,
    pub district:       String,
    pub city:           String,
    pub country:        String,
    pub delivery_note:  Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaymentSummaryDto {
    pub provider:    String,
    pub amount:      Decimal,
    pub currency:    String,
    pub status:      String,
    pub gateway_ref: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TrackingDto {
    pub carrier_code:    String,
    pub tracking_number: String,
    pub tracking_url:    Option<String>,
    pub last_status:     Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PlaceOrderResponse {
    pub order:        OrderSummaryDto,
    pub payment:      PaymentInstructionsDto,
}

#[derive(Debug, Serialize)]
pub struct PaymentInstructionsDto {
    pub payment_id:   Uuid,
    pub provider:     String,
    pub amount:       Decimal,
    pub currency:     String,
    pub instructions: serde_json::Value,
}
