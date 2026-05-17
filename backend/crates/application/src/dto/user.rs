use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id:           Uuid,
    pub email:        String,
    pub phone:        String,
    pub display_name: String,
    pub is_active:    bool,
    pub created_at:   DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserProfileDto {
    pub id:               Uuid,
    pub email:            String,
    pub phone:            String,
    pub display_name:     String,
    pub is_active:        bool,
    pub created_at:       DateTime<Utc>,
    pub membership:       Option<MembershipStatusDto>,
    pub credit_balance:   Decimal,
    pub addresses:        Vec<AddressDto>,
}

#[derive(Debug, Serialize)]
pub struct MembershipStatusDto {
    pub is_active:    bool,
    pub activated_at: DateTime<Utc>,
    pub expires_at:   Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct AddressDto {
    pub id:             Uuid,
    pub label:          String,
    pub recipient_name: String,
    pub phone:          String,
    pub street:         String,
    pub ward:           String,
    pub district:       String,
    pub city:           String,
    pub country:        String,
    pub is_default:     bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpsertAddressInput {
    pub id: Option<Uuid>,

    #[validate(length(min = 1, max = 50))]
    pub label: Option<String>,

    #[validate(length(min = 1, max = 100, message = "Recipient name required"))]
    pub recipient_name: String,

    #[validate(length(min = 9, max = 15))]
    pub phone: String,

    #[validate(length(min = 1, message = "Street required"))]
    pub street: String,

    #[validate(length(min = 1, message = "Ward required"))]
    pub ward: String,

    #[validate(length(min = 1, message = "District required"))]
    pub district: String,

    #[validate(length(min = 1, message = "City required"))]
    pub city: String,

    pub is_default: Option<bool>,
}
