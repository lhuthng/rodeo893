use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    entities::occasion::{Occasion, OccasionAnnouncement, OccasionProduct},
    error::DomainError,
};

#[async_trait]
pub trait OccasionRepository: Send + Sync {
    async fn list_active(&self) -> Result<Vec<Occasion>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Occasion, DomainError>;
    async fn create(&self, occasion: &Occasion) -> Result<Occasion, DomainError>;
    async fn update(&self, occasion: &Occasion) -> Result<Occasion, DomainError>;
}

#[async_trait]
pub trait OccasionProductRepository: Send + Sync {
    async fn list_for_occasion(&self, occasion_id: Uuid) -> Result<Vec<OccasionProduct>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<OccasionProduct, DomainError>;
    async fn create(&self, op: &OccasionProduct) -> Result<OccasionProduct, DomainError>;
    async fn reserve(&self, id: Uuid, qty: i32) -> Result<OccasionProduct, DomainError>;
    async fn release(&self, id: Uuid, qty: i32) -> Result<(), DomainError>;
}

#[async_trait]
pub trait OccasionAnnouncementRepository: Send + Sync {
    async fn create(&self, ann: &OccasionAnnouncement) -> Result<OccasionAnnouncement, DomainError>;
    async fn list_for_occasion(&self, occasion_id: Uuid) -> Result<Vec<OccasionAnnouncement>, DomainError>;
}
