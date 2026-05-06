use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::membership::{CreditLedgerEntry, Membership},
    repositories::membership::{CreditLedgerRepository, MembershipRepository},
    DomainError,
};
use rust_decimal::Decimal;

pub struct PgMembershipRepository(pub PgPool);

#[async_trait]
impl MembershipRepository for PgMembershipRepository {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<Membership>, DomainError> {
        sqlx::query_as!(Membership,
            "SELECT * FROM memberships WHERE user_id = $1", user_id
        )
        .fetch_optional(&self.0).await.map_err(db_err)
    }

    async fn create(&self, m: &Membership) -> Result<Membership, DomainError> {
        sqlx::query_as!(Membership,
            r#"INSERT INTO memberships (id,user_id,activated_at,expires_at,activated_by_payment_id,is_active)
               VALUES ($1,$2,$3,$4,$5,$6) RETURNING *"#,
            m.id, m.user_id, m.activated_at, m.expires_at, m.activated_by_payment_id, m.is_active
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn update(&self, m: &Membership) -> Result<Membership, DomainError> {
        sqlx::query_as!(Membership,
            "UPDATE memberships SET is_active=$2, expires_at=$3 WHERE id=$1 RETURNING *",
            m.id, m.is_active, m.expires_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }
}

pub struct PgCreditLedgerRepository(pub PgPool);

#[async_trait]
impl CreditLedgerRepository for PgCreditLedgerRepository {
    async fn append(&self, entry: &CreditLedgerEntry) -> Result<CreditLedgerEntry, DomainError> {
        sqlx::query_as!(CreditLedgerEntry,
            r#"INSERT INTO credit_ledger (id,user_id,order_id,amount,entry_type,description,created_at)
               VALUES ($1,$2,$3,$4,$5::credit_entry_type,$6,$7) RETURNING *"#,
            entry.id, entry.user_id, entry.order_id, entry.amount,
            entry.entry_type.as_str(), entry.description, entry.created_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn balance_for_user(&self, user_id: Uuid) -> Result<Decimal, DomainError> {
        let row = sqlx::query!(
            r#"SELECT COALESCE(SUM(
                CASE WHEN entry_type IN ('cashback','membership_topup') THEN amount ELSE -amount END
               ), 0) as balance FROM credit_ledger WHERE user_id = $1"#,
            user_id
        )
        .fetch_one(&self.0).await.map_err(db_err)?;
        Ok(row.balance.unwrap_or_default())
    }

    async fn cashback_exists_for_order(&self, order_id: Uuid) -> Result<bool, DomainError> {
        let row = sqlx::query!(
            "SELECT COUNT(*) as cnt FROM credit_ledger WHERE order_id=$1 AND entry_type='cashback'",
            order_id
        )
        .fetch_one(&self.0).await.map_err(db_err)?;
        Ok(row.cnt.unwrap_or(0) > 0)
    }

    async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<CreditLedgerEntry>, DomainError> {
        sqlx::query_as!(CreditLedgerEntry,
            "SELECT * FROM credit_ledger WHERE user_id=$1 ORDER BY created_at DESC", user_id
        )
        .fetch_all(&self.0).await.map_err(db_err)
    }
}

fn db_err(e: sqlx::Error) -> DomainError {
    match e {
        sqlx::Error::RowNotFound => DomainError::NotFound("Record not found".into()),
        sqlx::Error::Database(ref dbe) if dbe.code().as_deref() == Some("23505") =>
            DomainError::Conflict(dbe.message().to_string()),
        other => DomainError::Internal(other.to_string()),
    }
}
