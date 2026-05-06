use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::staff::{Staff, StaffRefreshToken},
    repositories::staff::{StaffRefreshTokenRepository, StaffRepository},
    DomainError,
};

pub struct PgStaffRepository(pub PgPool);

#[async_trait]
impl StaffRepository for PgStaffRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Staff, DomainError> {
        sqlx::query_as!(Staff,
            r#"SELECT id, name, email, password_hash, role as "role: _", is_active, created_at, updated_at
               FROM staff WHERE id = $1"#, id
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("Staff {}", id)))
    }

    async fn find_by_email(&self, email: &str) -> Result<Staff, DomainError> {
        sqlx::query_as!(Staff,
            r#"SELECT id, name, email, password_hash, role as "role: _", is_active, created_at, updated_at
               FROM staff WHERE email = $1"#, email
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound(format!("Staff email {}", email)))
    }
}

pub struct PgStaffRefreshTokenRepository(pub PgPool);

#[async_trait]
impl StaffRefreshTokenRepository for PgStaffRefreshTokenRepository {
    async fn create(&self, token: &StaffRefreshToken) -> Result<StaffRefreshToken, DomainError> {
        sqlx::query_as!(StaffRefreshToken,
            r#"INSERT INTO staff_refresh_tokens (id, staff_id, token_hash, expires_at, created_at)
               VALUES ($1,$2,$3,$4,$5) RETURNING *"#,
            token.id, token.staff_id, token.token_hash, token.expires_at, token.created_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_hash(&self, hash: &str) -> Result<StaffRefreshToken, DomainError> {
        sqlx::query_as!(StaffRefreshToken,
            "SELECT * FROM staff_refresh_tokens WHERE token_hash = $1", hash
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound("Refresh token".into()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM staff_refresh_tokens WHERE id = $1", id)
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
