use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TrackingStatusDto {
    pub carrier_code:    String,
    pub tracking_number: String,
    pub tracking_url:    Option<String>,
    pub last_status:     Option<String>,
    pub raw_status:      serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub provider_code: String,
    pub raw_body:      Vec<u8>,
    pub signature:     Option<String>,
}
