use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub id:                      Uuid,
    pub user_id:                 Uuid,
    pub activated_at:            DateTime<Utc>,
    /// None = lifetime membership
    pub expires_at:              Option<DateTime<Utc>>,
    pub activated_by_payment_id: Option<Uuid>,
    pub is_active:               bool,
    pub created_at:              DateTime<Utc>,
}

impl Membership {
    pub fn is_currently_active(&self) -> bool {
        if !self.is_active {
            return false;
        }
        match &self.expires_at {
            None => true,
            Some(exp) => Utc::now() < *exp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "credit_entry_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CreditEntryType {
    Cashback,
    Redemption,
    MembershipTopup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditLedgerEntry {
    pub id:          Uuid,
    pub user_id:     Uuid,
    pub order_id:    Option<Uuid>,
    pub amount:      Decimal,
    pub entry_type:  CreditEntryType,
    pub description: String,
    pub created_at:  DateTime<Utc>,
}
