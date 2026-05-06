use async_trait::async_trait;
use uuid::Uuid;
use crate::{entities::refund::{Refund, RefundStatus}, error::DomainError};

#[async_trait]
pub trait RefundRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Refund, DomainError>;
    async fn find_by_order(&self, order_id: Uuid) -> Result<Option<Refund>, DomainError>;
    async fn create(&self, refund: &Refund) -> Result<Refund, DomainError>;
    async fn update_status(&self, id: Uuid, status: RefundStatus) -> Result<Refund, DomainError>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Refund>, DomainError>;
}
