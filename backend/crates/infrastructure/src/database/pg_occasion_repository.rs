use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::occasion::{Occasion, OccasionAnnouncement, OccasionProduct},
    repositories::occasion::{OccasionAnnouncementRepository, OccasionProductRepository, OccasionRepository},
    DomainError,
};

pub struct PgOccasionRepository(pub PgPool);

#[async_trait]
impl OccasionRepository for PgOccasionRepository {
    async fn create(&self, o: &Occasion) -> Result<Occasion, DomainError> {
        sqlx::query_as!(Occasion,
            r#"INSERT INTO occasions (id,title,description,announcement_date,order_open_at,order_close_at,is_active,created_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *"#,
            o.id, o.title, o.description, o.announcement_date, o.order_open_at, o.order_close_at,
            o.is_active, o.created_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Occasion, DomainError> {
        sqlx::query_as!(Occasion, "SELECT * FROM occasions WHERE id=$1", id)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("Occasion {}", id)))
    }

    async fn list_active(&self) -> Result<Vec<Occasion>, DomainError> {
        sqlx::query_as!(Occasion, "SELECT * FROM occasions WHERE is_active=true ORDER BY order_open_at")
            .fetch_all(&self.0).await.map_err(db_err)
    }
}

pub struct PgOccasionProductRepository(pub PgPool);

#[async_trait]
impl OccasionProductRepository for PgOccasionProductRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<OccasionProduct, DomainError> {
        sqlx::query_as!(OccasionProduct, "SELECT * FROM occasion_products WHERE id=$1", id)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("OccasionProduct {}", id)))
    }

    async fn list_for_occasion(&self, occasion_id: Uuid) -> Result<Vec<OccasionProduct>, DomainError> {
        sqlx::query_as!(OccasionProduct,
            "SELECT * FROM occasion_products WHERE occasion_id=$1", occasion_id
        )
        .fetch_all(&self.0).await.map_err(db_err)
    }

    async fn reserve(&self, id: Uuid, qty: i32) -> Result<(), DomainError> {
        let rows = sqlx::query!(
            "UPDATE occasion_products SET reserved_qty = reserved_qty + $2 WHERE id=$1 AND reserved_qty + $2 <= total_qty",
            id, qty
        )
        .execute(&self.0).await.map_err(db_err)?;

        if rows.rows_affected() == 0 {
            let op = self.find_by_id(id).await?;
            return Err(DomainError::InsufficientStock {
                batch_id:  id,
                requested: qty,
                available: op.available_qty(),
            });
        }
        Ok(())
    }

    async fn release(&self, id: Uuid, qty: i32) -> Result<(), DomainError> {
        sqlx::query!(
            "UPDATE occasion_products SET reserved_qty = GREATEST(0, reserved_qty - $2) WHERE id=$1",
            id, qty
        )
        .execute(&self.0).await.map_err(db_err)?;
        Ok(())
    }
}

pub struct PgOccasionAnnouncementRepository(pub PgPool);

#[async_trait]
impl OccasionAnnouncementRepository for PgOccasionAnnouncementRepository {
    async fn create(&self, a: &OccasionAnnouncement) -> Result<OccasionAnnouncement, DomainError> {
        sqlx::query_as!(OccasionAnnouncement,
            r#"INSERT INTO occasion_announcements (id,occasion_id,sent_at,recipient_count,status)
               VALUES ($1,$2,$3,$4,$5) RETURNING *"#,
            a.id, a.occasion_id, a.sent_at, a.recipient_count, a.status
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }
}

fn db_err(e: sqlx::Error) -> DomainError {
    match e {
        sqlx::Error::RowNotFound => DomainError::NotFound("Record not found".into()),
        other => DomainError::Internal(other.to_string()),
    }
}
