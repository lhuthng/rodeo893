use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use domain::{
    entities::occasion::Occasion,
    repositories::occasion::{OccasionRepository},
};
use crate::error::AppError;

pub struct CreateOccasionInput {
    pub title:             String,
    pub description:       String,
    pub announcement_date: chrono::DateTime<chrono::Utc>,
    pub order_open_at:     chrono::DateTime<chrono::Utc>,
    pub order_close_at:    chrono::DateTime<chrono::Utc>,
}

pub struct CreateOccasion<OR> {
    pub occasion_repo: Arc<OR>,
}

impl<OR: OccasionRepository> CreateOccasion<OR> {
    pub async fn execute(&self, input: CreateOccasionInput) -> Result<Occasion, AppError> {
        if input.order_open_at >= input.order_close_at {
            return Err(AppError::Validation("order_open_at must be before order_close_at".into()));
        }

        let occasion = Occasion {
            id:                Uuid::new_v4(),
            title:             input.title,
            description:       input.description,
            announcement_date: input.announcement_date,
            order_open_at:     input.order_open_at,
            order_close_at:    input.order_close_at,
            is_active:         true,
            created_at:        Utc::now(),
        };

        Ok(self.occasion_repo.create(&occasion).await?)
    }
}
