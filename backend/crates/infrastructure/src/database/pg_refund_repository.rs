use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::refund::{Refund, RefundStatus},
    repositories::refund::RefundRepository,
    DomainError,
};

pub struct PgRefundRepository(pub PgPool);

#[async_trait]
impl RefundRepository for PgRefundRepository {
    async fn create(&self, r: &Refund) -> Result<Refund, DomainError> {
        sqlx::query_as!(Refund,
            r#"INSERT INTO refunds (id,payment_id,order_id,amount,reason,status,created_at,updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
               RETURNING id,payment_id,order_id,amount,reason,status as "status: RefundStatus",created_at,updated_at"#,
            r.id, r.payment_id, r.order_id, r.amount, r.reason,
            r.status as _, r.created_at, r.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Refund, DomainError> {
        sqlx::query_as!(Refund,
            r#"SELECT id,payment_id,order_id,amount,reason,status as "status: RefundStatus",created_at,updated_at
               FROM refunds WHERE id=$1"#, id
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("Refund {}", id)))
    }

    async fn find_by_order(&self, order_id: Uuid) -> Result<Option<Refund>, DomainError> {
        sqlx::query_as!(Refund,
            r#"SELECT id,payment_id,order_id,amount,reason,status as "status: RefundStatus",created_at,updated_at
               FROM refunds WHERE order_id=$1"#, order_id
        )
        .fetch_optional(&self.0).await.map_err(db_err)
    }

    async fn update_status(&self, id: Uuid, status: RefundStatus) -> Result<Refund, DomainError> {
        sqlx::query_as!(Refund,
            r#"UPDATE refunds SET status=$2, updated_at=NOW() WHERE id=$1
               RETURNING id,payment_id,order_id,amount,reason,status as "status: RefundStatus",created_at,updated_at"#,
            id, status as _
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Refund>, DomainError> {
        sqlx::query_as!(Refund,
            r#"SELECT id,payment_id,order_id,amount,reason,status as "status: RefundStatus",created_at,updated_at
               FROM refunds ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
            limit, offset
        )
        .fetch_all(&self.0).await.map_err(db_err)
    }
}

fn db_err(e: sqlx::Error) -> DomainError {
    match e {
        sqlx::Error::RowNotFound => DomainError::NotFound("Record not found".into()),
        other => DomainError::Internal(other.to_string()),
    }
}
