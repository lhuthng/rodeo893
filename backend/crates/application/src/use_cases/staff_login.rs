use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use domain::{
    entities::staff::StaffRefreshToken,
    repositories::staff::{StaffRepository, StaffRefreshTokenRepository},
    DomainError,
};
use crate::{
    dto::staff::{StaffAuthResponse, StaffLoginInput},
    error::AppError,
    use_cases::register_user::PasswordHasher,
};

/// Infrastructure provides this trait.
pub trait JwtService: Send + Sync {
    fn sign_access(&self, staff_id: Uuid, role: &str) -> Result<(String, i64), AppError>;
}

pub struct StaffLogin<SR, RR, H, J> {
    pub staff_repo:   Arc<SR>,
    pub refresh_repo: Arc<RR>,
    pub hasher:       Arc<H>,
    pub jwt:          Arc<J>,
}

impl<SR, RR, H, J> StaffLogin<SR, RR, H, J>
where
    SR: StaffRepository,
    RR: StaffRefreshTokenRepository,
    H:  PasswordHasher,
    J:  JwtService,
{
    pub async fn execute(&self, input: StaffLoginInput) -> Result<StaffAuthResponse, AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        let staff = self.staff_repo.find_by_email(&input.email).await
            .map_err(|e| match e {
                DomainError::NotFound(_) => AppError::Unauthorized("Invalid credentials".into()),
                other => other.into(),
            })?;

        if !staff.is_active {
            return Err(AppError::Unauthorized("Account disabled".into()));
        }

        let valid = self.hasher.verify(&input.password, &staff.password_hash).await?;
        if !valid {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }

        let role = format!("{:?}", staff.role).to_lowercase();
        let (access_token, expires_in) = self.jwt.sign_access(staff.id, &role)?;

        let refresh_raw = Uuid::new_v4().to_string().replace('-', "");
        let refresh_hash = sha256_hex(&refresh_raw);

        let rt = StaffRefreshToken {
            id:         Uuid::new_v4(),
            staff_id:   staff.id,
            token_hash: refresh_hash,
            expires_at: Utc::now() + chrono::Duration::days(7),
            created_at: Utc::now(),
        };
        self.refresh_repo.create(&rt).await?;

        Ok(StaffAuthResponse {
            access_token,
            refresh_token: refresh_raw,
            expires_in,
            staff_id:      staff.id,
            role,
        })
    }
}

fn sha256_hex(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}
