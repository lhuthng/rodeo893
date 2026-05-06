use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    entities::user::{User, UserSession, UserAddress},
    error::DomainError,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<User, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<User, DomainError>;
    async fn find_by_phone(&self, phone: &str) -> Result<User, DomainError>;
    async fn create(&self, user: &User) -> Result<User, DomainError>;
    async fn update(&self, user: &User) -> Result<User, DomainError>;
    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, DomainError>;
    async fn count(&self) -> Result<i64, DomainError>;
}

#[async_trait]
pub trait UserSessionRepository: Send + Sync {
    async fn create(&self, session: &UserSession) -> Result<(), DomainError>;
    async fn find_by_token_hash(&self, hash: &str) -> Result<UserSession, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_expired(&self) -> Result<u64, DomainError>;
}

#[async_trait]
pub trait UserAddressRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<UserAddress, DomainError>;
    async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<UserAddress>, DomainError>;
    async fn create(&self, address: &UserAddress) -> Result<UserAddress, DomainError>;
    async fn update(&self, address: &UserAddress) -> Result<UserAddress, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn set_default(&self, user_id: Uuid, address_id: Uuid) -> Result<(), DomainError>;
}
