use std::sync::Arc;
use domain::{
    entities::order::OrderStatus,
    repositories::order::{OrderFilters, OrderRepository},
};
use crate::{dto::order::OrderSummaryDto, error::AppError};

pub struct ListOrders<OR> {
    pub order_repo: Arc<OR>,
}

impl<OR: OrderRepository> ListOrders<OR> {
    pub async fn execute(
        &self,
        status: Option<String>,
        user_id: Option<uuid::Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<OrderSummaryDto>, i64), AppError> {
        let status_filter = match status.as_deref() {
            None                  => None,
            Some("pending_payment")   => Some(OrderStatus::PendingPayment),
            Some("confirmed")         => Some(OrderStatus::Confirmed),
            Some("processing")        => Some(OrderStatus::Processing),
            Some("out_for_delivery")  => Some(OrderStatus::OutForDelivery),
            Some("delivered")         => Some(OrderStatus::Delivered),
            Some("cancelled")         => Some(OrderStatus::Cancelled),
            Some("refunded")          => Some(OrderStatus::Refunded),
            Some(other) => return Err(AppError::Validation(format!("Unknown status: {}", other))),
        };

        let filters = OrderFilters { status: status_filter, user_id, limit, offset };
        let total = self.order_repo.count(&filters).await?;
        let orders = self.order_repo.list(filters).await?;

        let dtos = orders.into_iter().map(|o| OrderSummaryDto {
            id:             o.id,
            reference_code: o.reference_code,
            status:         o.status.to_string(),
            total_amount:   o.total_amount,
            currency:       o.currency,
            item_count:     0, // could be enriched
            created_at:     o.created_at,
        }).collect();

        Ok((dtos, total))
    }
}
