use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use domain::{
    entities::tracking::OrderTracking,
    repositories::tracking::TrackingRepository,
};
use crate::{dto::staff::AttachTrackingInput, error::AppError};

pub struct AttachTracking<TR> {
    pub tracking_repo: Arc<TR>,
}

impl<TR: TrackingRepository> AttachTracking<TR> {
    pub async fn execute(&self, order_id: Uuid, input: AttachTrackingInput) -> Result<(), AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        // Reject duplicate
        if self.tracking_repo.find_by_order(order_id).await?.is_some() {
            return Err(AppError::Conflict("Tracking already attached to this order".into()));
        }

        let now = Utc::now();
        let tracking = OrderTracking {
            id:              Uuid::new_v4(),
            order_id,
            carrier_code:    input.carrier_code,
            tracking_number: input.tracking_number,
            tracking_url:    input.tracking_url,
            last_status:     None,
            last_checked_at: None,
            raw_status:      serde_json::json!({}),
            created_at:      now,
            updated_at:      now,
        };
        self.tracking_repo.create(&tracking).await?;
        Ok(())
    }
}
