use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::stock::StockBatch,
    repositories::stock::StockBatchRepository,
    DomainError,
};

pub struct PgStockBatchRepository(pub PgPool);

#[async_trait]
impl StockBatchRepository for PgStockBatchRepository {
    async fn create(&self, b: &StockBatch) -> Result<StockBatch, DomainError> {
        sqlx::query_as!(StockBatch,
            r#"INSERT INTO stock_batches (id,product_id,week_start,total_qty,reserved_qty,is_released,created_at,updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *"#,
            b.id, b.product_id, b.week_start, b.total_qty, b.reserved_qty, b.is_released,
            b.created_at, b.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<StockBatch, DomainError> {
        sqlx::query_as!(StockBatch, "SELECT * FROM stock_batches WHERE id=$1", id)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("StockBatch {}", id)))
    }

    async fn find_for_week(&self, product_id: Uuid, week_start: NaiveDate) -> Result<Option<StockBatch>, DomainError> {
        sqlx::query_as!(StockBatch,
            "SELECT * FROM stock_batches WHERE product_id=$1 AND week_start=$2",
            product_id, week_start
        )
        .fetch_optional(&self.0).await.map_err(db_err)
    }

    async fn update(&self, b: &StockBatch) -> Result<StockBatch, DomainError> {
        sqlx::query_as!(StockBatch,
            r#"UPDATE stock_batches SET total_qty=$2,reserved_qty=$3,is_released=$4,updated_at=$5
               WHERE id=$1 RETURNING *"#,
            b.id, b.total_qty, b.reserved_qty, b.is_released, b.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    /// Atomic reserve: prevents over-reservation at the DB level.
    async fn reserve(&self, id: Uuid, qty: i32) -> Result<(), DomainError> {
        let rows = sqlx::query!(
            r#"UPDATE stock_batches SET reserved_qty = reserved_qty + $2
               WHERE id = $1 AND reserved_qty + $2 <= total_qty"#,
            id, qty
        )
        .execute(&self.0).await.map_err(db_err)?;

        if rows.rows_affected() == 0 {
            let b = self.find_by_id(id).await?;
            return Err(DomainError::InsufficientStock {
                batch_id:  id,
                requested: qty,
                available: b.available_qty(),
            });
        }
        Ok(())
    }

    async fn release(&self, id: Uuid, qty: i32) -> Result<(), DomainError> {
        sqlx::query!(
            "UPDATE stock_batches SET reserved_qty = GREATEST(0, reserved_qty - $2) WHERE id=$1",
            id, qty
        )
        .execute(&self.0).await.map_err(db_err)?;
        Ok(())
    }
}

fn db_err(e: sqlx::Error) -> DomainError {
    match e {
        sqlx::Error::RowNotFound => DomainError::NotFound("Record not found".into()),
        other => DomainError::Internal(other.to_string()),
    }
}
