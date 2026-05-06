use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString},
    Argon2,
};
use application::{error::AppError, use_cases::register_user::PasswordHasher};
use async_trait::async_trait;

pub struct Argon2Hasher;

#[async_trait]
impl PasswordHasher for Argon2Hasher {
    async fn hash(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(e.to_string()))?
            .to_string();
        Ok(hash)
    }

    async fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
    }
}
