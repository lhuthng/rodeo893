use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::notification::{EmailNotification, NotificationStatus},
    repositories::notification::EmailNotificationRepository,
    DomainError,
};

pub struct PgEmailNotificationRepository(pub PgPool);

#[async_trait]
impl EmailNotificationRepository for PgEmailNotificationRepository {
    async fn create(&self, n: &EmailNotification) -> Result<EmailNotification, DomainError> {
        sqlx::query_as!(EmailNotification,
            r#"INSERT INTO email_notifications (id,recipient_email,template_name,payload,status,error_text,scheduled_at,sent_at,created_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
               RETURNING id,recipient_email,template_name,payload,status as "status: NotificationStatus",error_text,scheduled_at,sent_at,created_at"#,
            n.id, n.recipient_email, n.template_name, n.payload,
            n.status as _, n.error_text, n.scheduled_at, n.sent_at, n.created_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn update(&self, n: &EmailNotification) -> Result<EmailNotification, DomainError> {
        sqlx::query_as!(EmailNotification,
            r#"UPDATE email_notifications SET status=$2,error_text=$3,sent_at=$4 WHERE id=$1
               RETURNING id,recipient_email,template_name,payload,status as "status: NotificationStatus",error_text,scheduled_at,sent_at,created_at"#,
            n.id, n.status as _, n.error_text, n.sent_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<EmailNotification, DomainError> {
        sqlx::query_as!(EmailNotification,
            r#"SELECT id,recipient_email,template_name,payload,status as "status: NotificationStatus",error_text,scheduled_at,sent_at,created_at
               FROM email_notifications WHERE id=$1"#, id
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("Notification {}", id)))
    }

    async fn list_pending(&self, limit: i64) -> Result<Vec<EmailNotification>, DomainError> {
        sqlx::query_as!(EmailNotification,
            r#"SELECT id,recipient_email,template_name,payload,status as "status: NotificationStatus",error_text,scheduled_at,sent_at,created_at
               FROM email_notifications WHERE status='pending'
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
