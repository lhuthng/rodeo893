use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use domain::{
    entities::user::{User, UserAddress, UserSession},
    repositories::user::{UserAddressRepository, UserRepository, UserSessionRepository},
    DomainError,
};

pub struct PgUserRepository(pub PgPool);

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(&self, user: &User) -> Result<User, DomainError> {
        let u = sqlx::query_as!(User,
            r#"INSERT INTO users (id, email, phone, display_name, password_hash, is_active, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *"#,
            user.id, user.email, user.phone, user.display_name, user.password_hash,
            user.is_active, user.created_at, user.updated_at
        )
        .fetch_one(&self.0).await
        .map_err(db_err)?;
        Ok(u)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<User, DomainError> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("User {}", id)))
    }

    async fn find_by_email(&self, email: &str) -> Result<User, DomainError> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("User email {}", email)))
    }

    async fn find_by_phone(&self, phone: &str) -> Result<User, DomainError> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE phone = $1", phone)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("User phone {}", phone)))
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        sqlx::query_as!(User,
            r#"UPDATE users SET display_name=$2, is_active=$3, updated_at=$4
               WHERE id=$1 RETURNING *"#,
            user.id, user.display_name, user.is_active, user.updated_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn list(&self, limit: i64, offset: i64) -> Result<Vec<User>, DomainError> {
        sqlx::query_as!(User,
            "SELECT * FROM users WHERE is_active = true ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit, offset
        )
        .fetch_all(&self.0).await.map_err(db_err)
    }
}

pub struct PgUserSessionRepository(pub PgPool);

#[async_trait]
impl UserSessionRepository for PgUserSessionRepository {
    async fn create(&self, session: &UserSession) -> Result<UserSession, DomainError> {
        sqlx::query_as!(UserSession,
            r#"INSERT INTO user_sessions (id, user_id, token_hash, expires_at, ip_addr, user_agent, created_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *"#,
            session.id, session.user_id, session.token_hash, session.expires_at,
            session.ip_addr, session.user_agent, session.created_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_token_hash(&self, hash: &str) -> Result<UserSession, DomainError> {
        sqlx::query_as!(UserSession,
            "SELECT * FROM user_sessions WHERE token_hash = $1", hash
        )
        .fetch_optional(&self.0).await.map_err(db_err)?
        .ok_or_else(|| DomainError::NotFound("Session".into()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM user_sessions WHERE id = $1", id)
            .execute(&self.0).await.map_err(db_err)?;
        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, DomainError> {
        let r = sqlx::query!("DELETE FROM user_sessions WHERE expires_at < NOW()")
            .execute(&self.0).await.map_err(db_err)?;
        Ok(r.rows_affected())
    }
}

pub struct PgUserAddressRepository(pub PgPool);

#[async_trait]
impl UserAddressRepository for PgUserAddressRepository {
    async fn create(&self, addr: &UserAddress) -> Result<UserAddress, DomainError> {
        sqlx::query_as!(UserAddress,
            r#"INSERT INTO user_addresses (id,user_id,label,recipient_name,phone,street,ward,district,city,country,is_default,created_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12) RETURNING *"#,
            addr.id, addr.user_id, addr.label, addr.recipient_name, addr.phone,
            addr.street, addr.ward, addr.district, addr.city, addr.country,
            addr.is_default, addr.created_at
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<UserAddress, DomainError> {
        sqlx::query_as!(UserAddress, "SELECT * FROM user_addresses WHERE id = $1", id)
            .fetch_optional(&self.0).await.map_err(db_err)?
            .ok_or_else(|| DomainError::NotFound(format!("Address {}", id)))
    }

    async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<UserAddress>, DomainError> {
        sqlx::query_as!(UserAddress,
            "SELECT * FROM user_addresses WHERE user_id = $1 ORDER BY is_default DESC, created_at DESC",
            user_id
        )
        .fetch_all(&self.0).await.map_err(db_err)
    }

    async fn update(&self, addr: &UserAddress) -> Result<UserAddress, DomainError> {
        sqlx::query_as!(UserAddress,
            r#"UPDATE user_addresses SET label=$2,recipient_name=$3,phone=$4,street=$5,
               ward=$6,district=$7,city=$8,is_default=$9 WHERE id=$1 RETURNING *"#,
            addr.id, addr.label, addr.recipient_name, addr.phone, addr.street,
            addr.ward, addr.district, addr.city, addr.is_default
        )
        .fetch_one(&self.0).await.map_err(db_err)
    }

    async fn clear_default(&self, user_id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("UPDATE user_addresses SET is_default=false WHERE user_id=$1", user_id)
            .execute(&self.0).await.map_err(db_err)?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM user_addresses WHERE id=$1", id)
            .execute(&self.0).await.map_err(db_err)?;
        Ok(())
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
