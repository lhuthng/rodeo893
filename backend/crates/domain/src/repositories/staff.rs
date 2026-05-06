use async_trait::async_trait;
use uuid::Uuid;
use crate::{entities::staff::{Staff, StaffRefreshToken}, error::DomainError};

#[async_trait]
pub trait StaffRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Staff, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Staff, DomainError>;
    async fn create(&self, staff: &Staff) -> Result<Staff, DomainError>;
    async fn update(&self, staff: &Staff) -> Result<Staff, DomainError>;
    async fn list(&self) -> Result<Vec<Staff>, DomainError>;
}

#[async_trait]
pub trait StaffRefreshTokenRepository: Send + Sync {
    async fn create(&self, token: &StaffRefreshToken) -> Result<(), DomainError>;
    async fn find_by_hash(&self, hash: &str) -> Result<StaffRefreshToken, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_all_for_staff(&self, staff_id: Uuid) -> Result<(), DomainError>;
}
