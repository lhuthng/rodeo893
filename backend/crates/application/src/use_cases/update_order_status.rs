use std::sync::Arc;
use uuid::Uuid;
use domain::{
    entities::order::OrderStatus,
    repositories::order::OrderRepository,
};
use crate::error::AppError;

pub struct UpdateOrderStatus<OR> {
    pub order_repo: Arc<OR>,
}

impl<OR: OrderRepository> UpdateOrderStatus<OR> {
    pub async fn execute(&self, order_id: Uuid, new_status: &str) -> Result<(), AppError> {
        let status = match new_status {
            "confirmed"        => OrderStatus::Confirmed,
            "processing"       => OrderStatus::Processing,
            "out_for_delivery" => OrderStatus::OutForDelivery,
            "delivered"        => OrderStatus::Delivered,
            "cancelled"        => OrderStatus::Cancelled,
            "refunded"         => OrderStatus::Refunded,
            other => return Err(AppError::Validation(format!("Unknown status: {}", other))),
        };
        self.order_repo.update_status(order_id, status).await?;
        Ok(())
    }
}
