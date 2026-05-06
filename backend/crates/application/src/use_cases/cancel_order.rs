use std::sync::Arc;
use uuid::Uuid;

use domain::{
    entities::order::OrderStatus,
    repositories::{
        order::{OrderRepository, OrderItemRepository},
        stock::StockBatchRepository,
        occasion::OccasionProductRepository,
    },
    DomainError,
};
use crate::error::AppError;

pub struct CancelOrder<OR, OIR, SBR, OPR> {
    pub order_repo:       Arc<OR>,
    pub item_repo:        Arc<OIR>,
    pub batch_repo:       Arc<SBR>,
    pub occ_product_repo: Arc<OPR>,
}

impl<OR, OIR, SBR, OPR> CancelOrder<OR, OIR, SBR, OPR>
where
    OR:  OrderRepository,
    OIR: OrderItemRepository,
    SBR: StockBatchRepository,
    OPR: OccasionProductRepository,
{
    /// Cancel an order. Returns the updated order. Caller should then initiate refund if paid.
    pub async fn execute(&self, order_id: Uuid, requesting_user_id: Option<Uuid>) -> Result<(), AppError> {
        let order = self.order_repo.find_by_id(order_id).await?;

        // Authorization: user can only cancel their own orders
        if let Some(uid) = requesting_user_id {
            if order.user_id != Some(uid) {
                return Err(AppError::Unauthorized("Not your order".into()));
            }
        }

        if !order.status.is_cancellable() {
            return Err(AppError::Domain(DomainError::OrderNotCancellable {
                reference: order.reference_code,
                status:    order.status.to_string(),
            }));
        }

        // Release reserved stock
        let items = self.item_repo.list_for_order(order_id).await?;
        for item in &items {
            if let Some(batch_id) = item.batch_id {
                self.batch_repo.release(batch_id, item.quantity).await?;
            }
            if let Some(occ_id) = item.occasion_product_id {
                self.occ_product_repo.release(occ_id, item.quantity).await?;
            }
        }

        self.order_repo.update_status(order_id, OrderStatus::Cancelled).await?;
        Ok(())
    }
}
