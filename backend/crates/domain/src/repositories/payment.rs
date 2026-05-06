use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    entities::payment::{Payment, PaymentEvent, PaymentProvider, PaymentStatus},
    error::DomainError,
};

#[async_trait]
pub trait PaymentProviderRepository: Send + Sync {
    async fn list_active(&self) -> Result<Vec<PaymentProvider>, DomainError>;
    async fn find_by_code(&self, code: &str) -> Result<PaymentProvider, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<PaymentProvider, DomainError>;
    async fn update(&self, provider: &PaymentProvider) -> Result<PaymentProvider, DomainError>;
}

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Payment, DomainError>;
    async fn find_by_order(&self, order_id: Uuid) -> Result<Payment, DomainError>;
    async fn find_by_gateway_ref(&self, gateway_ref: &str) -> Result<Payment, DomainError>;
    async fn create(&self, payment: &Payment) -> Result<Payment, DomainError>;
    async fn update_status(&self, id: Uuid, status: PaymentStatus, gateway_ref: Option<&str>) -> Result<Payment, DomainError>;
}

#[async_trait]
pub trait PaymentEventRepository: Send + Sync {
    async fn append(&self, event: &PaymentEvent) -> Result<PaymentEvent, DomainError>;
}
