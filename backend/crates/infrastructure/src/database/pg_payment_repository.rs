use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::payment::{Payment, PaymentEvent, PaymentProvider, PaymentStatus},
    repositories::payment::{PaymentEventRepository, PaymentProviderRepository, PaymentRepository},
    DomainError,
};

pub struct PgPaymentProviderRepository(pub PgPool);

#[async_trait]
impl PaymentProviderRepository for PgPaymentProviderRepository {
    async fn find_by_code(&self, code: &str) -> Result<PaymentProvider, DomainError> {
        sqlx::query_as!(PaymentProvider, "SELECT * FROM payment_providers WHERE code=$1", code)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("Provider {}", code)))
    }

    async fn list_active(&self) -> Result<Vec<PaymentProvider>, DomainError> {
        sqlx::query_as!(PaymentProvider, "SELECT * FROM payment_providers WHERE is_active=true")
            .fetch_all(&self.0).await.map_err(db_err)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<PaymentProvider, DomainError> {
        sqlx::query_as!(PaymentProvider, "SELECT * FROM payment_providers WHERE id=$1", id)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("Provider {}", id)))
    }

    async fn update(&self, p: &PaymentProvider) -> Result<PaymentProvider, DomainError> {
        sqlx::query_as!(PaymentProvider,
            "UPDATE payment_providers SET name=$2,is_active=$3,config_json=$4 WHERE id=$1 RETURNING *",
            p.id, p.name, p.is_active, p.config_json
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }
}

pub struct PgPaymentRepository(pub PgPool);

#[async_trait]
impl PaymentRepository for PgPaymentRepository {
    async fn create(&self, p: &Payment) -> Result<Payment, DomainError> {
        sqlx::query_as!(Payment,
            r#"INSERT INTO payments (id,order_id,provider_id,amount,currency,status,gateway_ref,gateway_payload,created_at,updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
               RETURNING id,order_id,provider_id,amount,currency,status as "status: PaymentStatus",gateway_ref,gateway_payload,created_at,updated_at"#,
            p.id, p.order_id, p.provider_id, p.amount, p.currency,
            p.status as _, p.gateway_ref, p.gateway_payload, p.created_at, p.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Payment, DomainError> {
        sqlx::query_as!(Payment,
            r#"SELECT id,order_id,provider_id,amount,currency,status as "status: PaymentStatus",gateway_ref,gateway_payload,created_at,updated_at
               FROM payments WHERE id=$1"#, id
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("Payment {}", id)))
    }

    async fn find_by_order(&self, order_id: Uuid) -> Result<Payment, DomainError> {
        sqlx::query_as!(Payment,
            r#"SELECT id,order_id,provider_id,amount,currency,status as "status: PaymentStatus",gateway_ref,gateway_payload,created_at,updated_at
               FROM payments WHERE order_id=$1"#, order_id
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("Payment for order {}", order_id)))
    }

    async fn find_by_gateway_ref(&self, gateway_ref: &str) -> Result<Payment, DomainError> {
        sqlx::query_as!(Payment,
            r#"SELECT id,order_id,provider_id,amount,currency,status as "status: PaymentStatus",gateway_ref,gateway_payload,created_at,updated_at
               FROM payments WHERE gateway_ref=$1"#, gateway_ref
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("Payment ref {}", gateway_ref)))
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: PaymentStatus,
        gateway_ref: Option<&str>,
    ) -> Result<Payment, DomainError> {
        sqlx::query_as!(Payment,
            r#"UPDATE payments SET status=$2, gateway_ref=COALESCE($3, gateway_ref), updated_at=NOW() WHERE id=$1
               RETURNING id,order_id,provider_id,amount,currency,status as "status: PaymentStatus",gateway_ref,gateway_payload,created_at,updated_at"#,
            id, status as _, gateway_ref
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }
}

pub struct PgPaymentEventRepository(pub PgPool);

#[async_trait]
impl PaymentEventRepository for PgPaymentEventRepository {
    async fn append(&self, event: &PaymentEvent) -> Result<PaymentEvent, DomainError> {
        sqlx::query_as!(PaymentEvent,
            r#"INSERT INTO payment_events (id,payment_id,event_type,raw_payload,received_at)
               VALUES ($1,$2,$3,$4,$5) RETURNING *"#,
            event.id, event.payment_id, event.event_type, event.raw_payload, event.received_at
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
