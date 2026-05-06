use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use domain::repositories::staff::{StaffRepository, StaffRefreshTokenRepository};
use crate::{dto::staff::StaffAuthResponse, error::AppError, use_cases::staff_login::JwtService};

pub struct StaffRefresh<SR, RR, J> {
    pub staff_repo:   Arc<SR>,
    pub refresh_repo: Arc<RR>,
    pub jwt:          Arc<J>,
}

impl<SR, RR, J> StaffRefresh<SR, RR, J>
where
    SR: StaffRepository,
    RR: StaffRefreshTokenRepository,
    J:  JwtService,
{
    pub async fn execute(&self, refresh_token_raw: &str) -> Result<StaffAuthResponse, AppError> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(refresh_token_raw.as_bytes());
        let hash = hex::encode(hasher.finalize());

        let rt = self.refresh_repo.find_by_hash(&hash).await
            .map_err(|_| AppError::Unauthorized("Invalid refresh token".into()))?;

        if Utc::now() > rt.expires_at {
            self.refresh_repo.delete(rt.id).await?;
            return Err(AppError::Unauthorized("Refresh token expired".into()));
        }

        let staff = self.staff_repo.find_by_id(rt.staff_id).await?;
        if !staff.is_active {
            return Err(AppError::Unauthorized("Account disabled".into()));
        }

        let role = format!("{:?}", staff.role).to_lowercase();
        let (access_token, expires_in) = self.jwt.sign_access(staff.id, &role)?;

        // Rotate refresh token
        self.refresh_repo.delete(rt.id).await?;
        let new_raw = Uuid::new_v4().to_string().replace('-', "");
        let new_hash = {
            let mut h = Sha256::new();
            h.update(new_raw.as_bytes());
            hex::encode(h.finalize())
        };
        let new_rt = domain::entities::staff::StaffRefreshToken {
            id:         Uuid::new_v4(),
            staff_id:   staff.id,
            token_hash: new_hash,
            expires_at: Utc::now() + chrono::Duration::days(7),
            created_at: Utc::now(),
        };
        self.refresh_repo.create(&new_rt).await?;

        Ok(StaffAuthResponse {
            access_token,
            refresh_token: new_raw,
            expires_in,
            staff_id:      staff.id,
            role,
        })
    }
}
