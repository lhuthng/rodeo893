use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use domain::{repositories::user::{UserRepository, UserSessionRepository}, DomainError};
use crate::{
    dto::{auth::{AuthResponse, LoginInput}, user::UserDto},
    error::AppError,
    use_cases::register_user::PasswordHasher,
};

pub struct LoginUser<UR, SR, H> {
    pub user_repo:    Arc<UR>,
    pub session_repo: Arc<SR>,
    pub hasher:       Arc<H>,
}

impl<UR, SR, H> LoginUser<UR, SR, H>
where
    UR: UserRepository,
    SR: UserSessionRepository,
    H:  PasswordHasher,
{
    pub async fn execute(
        &self,
        input: LoginInput,
        ip_addr: Option<String>,
        user_agent: Option<String>,
    ) -> Result<AuthResponse, AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        let user = self.user_repo.find_by_email(&input.email).await
            .map_err(|e| match e {
                DomainError::NotFound(_) => AppError::Unauthorized("Invalid credentials".into()),
                other => other.into(),
            })?;

        if !user.is_active {
            return Err(AppError::Unauthorized("Account is disabled".into()));
        }

        let valid = self.hasher.verify(&input.password, &user.password_hash).await?;
        if !valid {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }

        let token = Uuid::new_v4().to_string().replace('-', "");
        let token_hash = sha256_hex(&token);
        let expires_at = Utc::now() + chrono::Duration::days(30);

        let session = domain::entities::user::UserSession {
            id:         Uuid::new_v4(),
            user_id:    user.id,
            token_hash,
            expires_at,
            created_at: Utc::now(),
            ip_addr,
            user_agent,
        };
        self.session_repo.create(&session).await?;

        Ok(AuthResponse {
            token,
            expires_at,
            user: UserDto {
                id:           user.id,
                email:        user.email,
                phone:        user.phone,
                display_name: user.display_name,
                is_active:    user.is_active,
                created_at:   user.created_at,
            },
        })
    }
}

fn sha256_hex(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}
