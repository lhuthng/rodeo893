use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::{
    entities::membership::{CreditLedgerEntry, Membership},
    error::DomainError,
};

#[async_trait]
pub trait MembershipRepository: Send + Sync {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<Membership>, DomainError>;
    async fn create(&self, membership: &Membership) -> Result<Membership, DomainError>;
    async fn update(&self, membership: &Membership) -> Result<Membership, DomainError>;
}

#[async_trait]
pub trait CreditLedgerRepository: Send + Sync {
    async fn append(&self, entry: &CreditLedgerEntry) -> Result<CreditLedgerEntry, DomainError>;
    async fn balance_for_user(&self, user_id: Uuid) -> Result<Decimal, DomainError>;
    async fn list_for_user(&self, user_id: Uuid, limit: i64, offset: i64) -> Result<Vec<CreditLedgerEntry>, DomainError>;
    /// Idempotency check: returns true if a cashback entry already exists for order_id
    async fn cashback_exists_for_order(&self, order_id: Uuid) -> Result<bool, DomainError>;
}
