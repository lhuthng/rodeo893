use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use domain::{
    entities::user::User,
    repositories::user::{UserRepository, UserSessionRepository},
    DomainError,
};
use crate::{
    dto::auth::{AuthResponse, RegisterUserInput},
    dto::user::UserDto,
    error::AppError,
};

/// Hashes password and creates a new user + session in a single call.
/// Returns a session token (raw, unhashed) for immediate use.
pub struct RegisterUser<UR, SR, H> {
    pub user_repo:    Arc<UR>,
    pub session_repo: Arc<SR>,
    pub hasher:       Arc<H>,
}

/// Minimal trait boundary so infrastructure can inject any hasher.
#[async_trait::async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: &str) -> Result<String, AppError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError>;
}

/// Minimal trait for session token generation/hashing.
pub trait SessionTokenService: Send + Sync {
    fn generate(&self) -> String;
    fn hash(&self, token: &str) -> String;
    fn expires_at(&self) -> chrono::DateTime<chrono::Utc>;
}

impl<UR, SR, H> RegisterUser<UR, SR, H>
where
    UR: UserRepository,
    SR: UserSessionRepository,
    H:  PasswordHasher,
{
    pub async fn execute(
        &self,
        input: RegisterUserInput,
        ip_addr: Option<String>,
        user_agent: Option<String>,
    ) -> Result<AuthResponse, AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        // Check uniqueness
        match self.user_repo.find_by_email(&input.email).await {
            Ok(_) => return Err(AppError::Conflict("Email already registered".into())),
            Err(DomainError::NotFound(_)) => {}
            Err(e) => return Err(e.into()),
        }
        match self.user_repo.find_by_phone(&input.phone).await {
            Ok(_) => return Err(AppError::Conflict("Phone already registered".into())),
            Err(DomainError::NotFound(_)) => {}
            Err(e) => return Err(e.into()),
        }

        let password_hash = self.hasher.hash(&input.password).await?;
        let now = Utc::now();

        let user = User {
            id:            Uuid::new_v4(),
            email:         input.email,
            phone:         input.phone,
            display_name:  input.display_name,
            password_hash,
            is_active:     true,
            created_at:    now,
            updated_at:    now,
        };

        let user = self.user_repo.create(&user).await?;

        // Issue session
        let token = uuid::Uuid::new_v4().to_string().replace('-', "");
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
