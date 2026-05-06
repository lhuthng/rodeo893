use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    entities::order::{Order, OrderDeliveryInfo, OrderItem, OrderStatus},
    error::DomainError,
};

pub struct OrderFilters {
    pub status:   Option<OrderStatus>,
    pub user_id:  Option<Uuid>,
    pub limit:    i64,
    pub offset:   i64,
}

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Order, DomainError>;
    async fn find_by_reference(&self, reference: &str) -> Result<Order, DomainError>;
    async fn list(&self, filters: OrderFilters) -> Result<Vec<Order>, DomainError>;
    async fn count(&self, filters: &OrderFilters) -> Result<i64, DomainError>;
    async fn create(&self, order: &Order) -> Result<Order, DomainError>;
    async fn update_status(&self, id: Uuid, status: OrderStatus) -> Result<Order, DomainError>;
}

#[async_trait]
pub trait OrderItemRepository: Send + Sync {
    async fn create_many(&self, items: &[OrderItem]) -> Result<Vec<OrderItem>, DomainError>;
    async fn list_for_order(&self, order_id: Uuid) -> Result<Vec<OrderItem>, DomainError>;
}

#[async_trait]
pub trait OrderDeliveryInfoRepository: Send + Sync {
    async fn create(&self, info: &OrderDeliveryInfo) -> Result<OrderDeliveryInfo, DomainError>;
    async fn find_by_order(&self, order_id: Uuid) -> Result<OrderDeliveryInfo, DomainError>;
    async fn update(&self, info: &OrderDeliveryInfo) -> Result<OrderDeliveryInfo, DomainError>;
}
