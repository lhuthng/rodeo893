use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use domain::{
    entities::{payment::PaymentStatus, refund::{Refund, RefundStatus}},
    repositories::{
        order::OrderRepository,
        payment::PaymentRepository,
        refund::RefundRepository,
    },
    DomainError,
};
use crate::error::AppError;

pub struct ProcessRefund<OR, PR, RR> {
    pub order_repo:   Arc<OR>,
    pub payment_repo: Arc<PR>,
    pub refund_repo:  Arc<RR>,
}

impl<OR, PR, RR> ProcessRefund<OR, PR, RR>
where
    OR: OrderRepository,
    PR: PaymentRepository,
    RR: RefundRepository,
{
    pub async fn request(
        &self,
        order_id: Uuid,
        reason: String,
    ) -> Result<Refund, AppError> {
        let payment = self.payment_repo.find_by_order(order_id).await
            .map_err(|e| match e {
                DomainError::NotFound(_) => AppError::Conflict("No payment found for order".into()),
                other => other.into(),
            })?;

        if payment.status != PaymentStatus::Completed {
            return Err(AppError::Conflict("Payment not completed; cannot refund".into()));
        }

        // Prevent duplicate refund requests
        if let Some(existing) = self.refund_repo.find_by_order(order_id).await? {
            return Err(AppError::Conflict(format!(
                "Refund already exists in status: {}", existing.status
            )));
        }

        let now = Utc::now();
        let refund = Refund {
            id:         Uuid::new_v4(),
            payment_id: payment.id,
            order_id,
            amount:     payment.amount,
            reason,
            status:     RefundStatus::Pending,
            created_at: now,
            updated_at: now,
        };

        Ok(self.refund_repo.create(&refund).await?)
    }

    pub async fn approve(&self, refund_id: Uuid) -> Result<Refund, AppError> {
        let refund = self.refund_repo.find_by_id(refund_id).await?;
        if refund.status != RefundStatus::Pending {
            return Err(AppError::Conflict(format!(
                "Refund is not pending (status: {})", refund.status
            )));
        }
        Ok(self.refund_repo.update_status(refund_id, RefundStatus::Approved).await?)
    }

    pub async fn reject(&self, refund_id: Uuid) -> Result<Refund, AppError> {
        let refund = self.refund_repo.find_by_id(refund_id).await?;
        if refund.status != RefundStatus::Pending {
            return Err(AppError::Conflict(format!(
                "Refund is not pending (status: {})", refund.status
            )));
        }
        Ok(self.refund_repo.update_status(refund_id, RefundStatus::Rejected).await?)
    }
}
