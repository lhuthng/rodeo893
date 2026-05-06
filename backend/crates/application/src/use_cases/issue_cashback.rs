use std::sync::Arc;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use domain::{
    entities::membership::{CreditEntryType, CreditLedgerEntry},
    entities::order::OrderStatus,
    repositories::{
        membership::{CreditLedgerRepository, MembershipRepository},
        order::OrderRepository,
    },
};
use crate::error::AppError;

/// Issued as an Apalis job when order transitions to 'delivered'.
/// Idempotent: safe to retry — checks for existing cashback entry for order_id.
pub struct IssueCashback<OR, MR, CR> {
    pub order_repo:      Arc<OR>,
    pub membership_repo: Arc<MR>,
    pub credit_repo:     Arc<CR>,
}

impl<OR, MR, CR> IssueCashback<OR, MR, CR>
where
    OR: OrderRepository,
    MR: MembershipRepository,
    CR: CreditLedgerRepository,
{
    pub async fn execute(&self, order_id: Uuid) -> Result<(), AppError> {
        let order = self.order_repo.find_by_id(order_id).await?;

        // Only issue cashback for delivered orders
        if order.status != OrderStatus::Delivered {
            return Ok(());
        }

        // Guest orders have no user, skip
        let user_id = match order.user_id {
            Some(id) => id,
            None => return Ok(()),
        };

        // Idempotency: skip if cashback already issued
        if self.credit_repo.cashback_exists_for_order(order_id).await? {
            return Ok(());
        }

        // Only issue if user has active membership
        let membership = self.membership_repo.find_by_user_id(user_id).await?;
        let is_member = membership.map(|m| m.is_currently_active()).unwrap_or(false);
        if !is_member {
            return Ok(());
        }

        let cashback_amount = (order.total_amount
            * Decimal::new(10, 2))   // 10%
            .round_dp(0);

        let entry = CreditLedgerEntry {
            id:          Uuid::new_v4(),
            user_id,
            order_id:    Some(order_id),
            amount:      cashback_amount,
            entry_type:  CreditEntryType::Cashback,
            description: format!("10% cashback on order {}", order.reference_code),
            created_at:  Utc::now(),
        };
        self.credit_repo.append(&entry).await?;

        Ok(())
    }
}
