use std::sync::Arc;
use chrono::{Datelike, NaiveDate, Weekday};
use uuid::Uuid;
use validator::Validate;

use domain::{entities::stock::StockBatch, repositories::stock::StockBatchRepository};
use crate::{dto::product::{CreateStockBatchInput, StockBatchDto}, error::AppError};

pub struct CreateStockBatch<SBR> {
    pub batch_repo: Arc<SBR>,
}

impl<SBR: StockBatchRepository> CreateStockBatch<SBR> {
    pub async fn execute(&self, input: CreateStockBatchInput) -> Result<StockBatchDto, AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        // Enforce week_start is a Monday
        if input.week_start.weekday() != Weekday::Mon {
            return Err(AppError::Validation("week_start must be a Monday".into()));
        }

        // Idempotency: reject duplicate batch for same product+week
        if let Some(_) = self.batch_repo.find_for_week(input.product_id, input.week_start).await? {
            return Err(AppError::Conflict(format!(
                "Batch already exists for product {} week {}",
                input.product_id, input.week_start
            )));
        }

        let now = chrono::Utc::now();
        let batch = StockBatch {
            id:           Uuid::new_v4(),
            product_id:   input.product_id,
            week_start:   input.week_start,
            total_qty:    input.total_qty,
            reserved_qty: 0,
            is_released:  false,
            created_at:   now,
            updated_at:   now,
        };

        let batch = self.batch_repo.create(&batch).await?;
        Ok(to_dto(batch))
    }
}

pub fn to_dto(b: StockBatch) -> StockBatchDto {
    let available = b.available_qty();
    StockBatchDto {
        id:            b.id,
        product_id:    b.product_id,
        week_start:    b.week_start,
        total_qty:     b.total_qty,
        reserved_qty:  b.reserved_qty,
        available_qty: available,
        is_released:   b.is_released,
        created_at:    b.created_at,
    }
}
