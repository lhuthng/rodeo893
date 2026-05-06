use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    entities::product::{Product, ProductCategory, ProductOrderingDay},
    error::DomainError,
};

#[async_trait]
pub trait ProductCategoryRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<ProductCategory>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<ProductCategory, DomainError>;
    async fn find_by_slug(&self, slug: &str) -> Result<ProductCategory, DomainError>;
    async fn create(&self, category: &ProductCategory) -> Result<ProductCategory, DomainError>;
    async fn update(&self, category: &ProductCategory) -> Result<ProductCategory, DomainError>;
}

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn list_active(&self) -> Result<Vec<Product>, DomainError>;
    async fn list_by_category(&self, category_id: Uuid) -> Result<Vec<Product>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Product, DomainError>;
    async fn find_by_slug(&self, slug: &str) -> Result<Product, DomainError>;
    async fn create(&self, product: &Product) -> Result<Product, DomainError>;
    async fn update(&self, product: &Product) -> Result<Product, DomainError>;
    /// Returns Err(Conflict) if product has open orders
    async fn deactivate(&self, id: Uuid) -> Result<(), DomainError>;
    async fn has_open_orders(&self, id: Uuid) -> Result<bool, DomainError>;
}

#[async_trait]
pub trait ProductOrderingDayRepository: Send + Sync {
    async fn list_for_product(&self, product_id: Uuid) -> Result<Vec<ProductOrderingDay>, DomainError>;
    async fn set_for_product(&self, product_id: Uuid, days: &[i16]) -> Result<(), DomainError>;
}
