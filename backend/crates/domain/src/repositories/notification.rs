use async_trait::async_trait;
use uuid::Uuid;
use crate::{entities::notification::EmailNotification, error::DomainError};

#[async_trait]
pub trait EmailNotificationRepository: Send + Sync {
    async fn create(&self, notification: &EmailNotification) -> Result<EmailNotification, DomainError>;
    async fn update(&self, notification: &EmailNotification) -> Result<EmailNotification, DomainError>;
    async fn list_pending(&self, limit: i64) -> Result<Vec<EmailNotification>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<EmailNotification, DomainError>;
}
