use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::tracking::OrderTracking,
    repositories::tracking::TrackingRepository,
    DomainError,
};

pub struct PgTrackingRepository(pub PgPool);

#[async_trait]
impl TrackingRepository for PgTrackingRepository {
    async fn create(&self, t: &OrderTracking) -> Result<OrderTracking, DomainError> {
        sqlx::query_as!(OrderTracking,
            r#"INSERT INTO order_tracking (id,order_id,carrier_code,tracking_number,tracking_url,last_status,last_checked_at,raw_status,created_at,updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) RETURNING *"#,
            t.id, t.order_id, t.carrier_code, t.tracking_number, t.tracking_url,
            t.last_status, t.last_checked_at, t.raw_status, t.created_at, t.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_order(&self, order_id: Uuid) -> Result<Option<OrderTracking>, DomainError> {
        sqlx::query_as!(OrderTracking,
            "SELECT * FROM order_tracking WHERE order_id=$1", order_id
        )
        .fetch_optional(&self.0).await.map_err(db_err)
    }

    async fn update(&self, t: &OrderTracking) -> Result<OrderTracking, DomainError> {
        sqlx::query_as!(OrderTracking,
            r#"UPDATE order_tracking SET last_status=$2,last_checked_at=$3,raw_status=$4,updated_at=$5
               WHERE id=$1 RETURNING *"#,
            t.id, t.last_status, t.last_checked_at, t.raw_status, t.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    /// Returns tracking entries not checked in the last 5 minutes.
    async fn list_stale(&self, limit: i64) -> Result<Vec<OrderTracking>, DomainError> {
        sqlx::query_as!(OrderTracking,
            r#"SELECT * FROM order_tracking
               WHERE last_checked_at IS NULL OR last_checked_at < NOW() - INTERVAL '5 minutes'
               LIMIT $1"#,
            limit
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
