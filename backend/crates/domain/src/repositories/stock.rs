use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use crate::{entities::stock::StockBatch, error::DomainError};

#[async_trait]
pub trait StockBatchRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<StockBatch, DomainError>;
    async fn find_for_week(&self, product_id: Uuid, week_start: NaiveDate) -> Result<Option<StockBatch>, DomainError>;
    async fn list_by_week(&self, week_start: NaiveDate) -> Result<Vec<StockBatch>, DomainError>;
    async fn create(&self, batch: &StockBatch) -> Result<StockBatch, DomainError>;
    async fn update(&self, batch: &StockBatch) -> Result<StockBatch, DomainError>;
    /// Atomically increments reserved_qty. Returns Err(InsufficientStock) if not enough.
    async fn reserve(&self, batch_id: Uuid, qty: i32) -> Result<StockBatch, DomainError>;
    /// Atomically decrements reserved_qty (for cancellations).
    async fn release(&self, batch_id: Uuid, qty: i32) -> Result<(), DomainError>;
}
