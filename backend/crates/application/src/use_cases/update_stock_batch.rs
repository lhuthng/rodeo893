use std::sync::Arc;
use uuid::Uuid;
use domain::repositories::stock::StockBatchRepository;
use crate::{dto::product::StockBatchDto, error::AppError};
use super::create_stock_batch::to_dto;

pub struct UpdateStockBatch<SBR> {
    pub batch_repo: Arc<SBR>,
}

impl<SBR: StockBatchRepository> UpdateStockBatch<SBR> {
    /// Update total_qty (must not go below reserved_qty).
    pub async fn execute(&self, batch_id: Uuid, new_total_qty: i32) -> Result<StockBatchDto, AppError> {
        let mut batch = self.batch_repo.find_by_id(batch_id).await?;

        if new_total_qty < batch.reserved_qty {
            return Err(AppError::Conflict(format!(
                "Cannot reduce total_qty to {} — {} already reserved",
                new_total_qty, batch.reserved_qty
            )));
        }

        batch.total_qty  = new_total_qty;
        batch.updated_at = chrono::Utc::now();
        let batch = self.batch_repo.update(&batch).await?;
        Ok(to_dto(batch))
    }
}
