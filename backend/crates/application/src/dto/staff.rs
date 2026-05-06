use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AttachTrackingInput {
    #[validate(length(min = 1))]
    pub carrier_code:    String,

    #[validate(length(min = 1))]
    pub tracking_number: String,

    pub tracking_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrderStatusInput {
    #[validate(length(min = 1))]
    pub status: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct StaffLoginInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct StaffAuthResponse {
    pub access_token:  String,
    pub refresh_token: String,
    pub expires_in:    i64,
    pub staff_id:      Uuid,
    pub role:          String,
}
