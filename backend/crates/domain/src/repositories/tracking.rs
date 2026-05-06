use async_trait::async_trait;
use uuid::Uuid;
use crate::{entities::tracking::OrderTracking, error::DomainError};

#[async_trait]
pub trait TrackingRepository: Send + Sync {
    async fn find_by_order(&self, order_id: Uuid) -> Result<Option<OrderTracking>, DomainError>;
    async fn create(&self, tracking: &OrderTracking) -> Result<OrderTracking, DomainError>;
    async fn update(&self, tracking: &OrderTracking) -> Result<OrderTracking, DomainError>;
    async fn list_stale(&self, older_than_minutes: i64) -> Result<Vec<OrderTracking>, DomainError>;
}
