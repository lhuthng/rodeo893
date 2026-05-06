use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Job payload for issuing cashback on delivered orders.
/// Enqueue this via apalis when an order transitions to 'delivered'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCashbackJob {
    pub order_id: Uuid,
}
