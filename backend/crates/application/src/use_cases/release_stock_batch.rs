use std::sync::Arc;
use uuid::Uuid;
use domain::repositories::stock::StockBatchRepository;
use crate::{dto::product::StockBatchDto, error::AppError};
use super::create_stock_batch::to_dto;

pub struct ReleaseStockBatch<SBR> {
    pub batch_repo: Arc<SBR>,
}

impl<SBR: StockBatchRepository> ReleaseStockBatch<SBR> {
    pub async fn execute(&self, batch_id: Uuid) -> Result<StockBatchDto, AppError> {
        let mut batch = self.batch_repo.find_by_id(batch_id).await?;

        if batch.is_released {
            return Err(AppError::Conflict("Batch already released".into()));
        }

        batch.is_released = true;
        batch.updated_at  = chrono::Utc::now();
        let batch = self.batch_repo.update(&batch).await?;
        Ok(to_dto(batch))
    }
}
