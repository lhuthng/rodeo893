use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::notification::EmailNotification,
    repositories::notification::EmailNotificationRepository,
    DomainError,
};

pub struct PgEmailNotificationRepository(pub PgPool);

#[async_trait]
impl EmailNotificationRepository for PgEmailNotificationRepository {
    async fn create(&self, n: &EmailNotification) -> Result<EmailNotification, DomainError> {
        sqlx::query_as!(EmailNotification,
            r#"INSERT INTO email_notifications (id,recipient_email,template_name,payload,status,error_text,scheduled_at,sent_at,created_at)
               VALUES ($1,$2,$3,$4,$5::notification_status,$6,$7,$8,$9) RETURNING *"#,
            n.id, n.recipient_email, n.template_name, n.payload,
            n.status.as_str(), n.error_text, n.scheduled_at, n.sent_at, n.created_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn mark_sent(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!(
            "UPDATE email_notifications SET status='sent'::notification_status, sent_at=NOW() WHERE id=$1",
            id
        )
        .execute(&self.0).await.map_err(db_err)?;
        Ok(())
    }

    async fn mark_failed(&self, id: Uuid, error: &str) -> Result<(), DomainError> {
        sqlx::query!(
            "UPDATE email_notifications SET status='failed'::notification_status, error_text=$2 WHERE id=$1",
            id, error
        )
        .execute(&self.0).await.map_err(db_err)?;
        Ok(())
    }

    async fn list_pending(&self, limit: i64) -> Result<Vec<EmailNotification>, DomainError> {
        sqlx::query_as!(EmailNotification,
            r#"SELECT * FROM email_notifications WHERE status='pending'::notification_status
               AND scheduled_at <= NOW() ORDER BY scheduled_at LIMIT $1"#,
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
