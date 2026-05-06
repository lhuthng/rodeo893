use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use application::{error::AppError, use_cases::staff_login::JwtService};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub:  String,
    pub role: String,
    pub exp:  i64,
    pub iat:  i64,
}

pub struct JwtServiceImpl {
    pub secret: String,
}

impl JwtService for JwtServiceImpl {
    fn sign_access(&self, staff_id: Uuid, role: &str) -> Result<(String, i64), AppError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AppError::Internal(e.to_string()))?
            .as_secs() as i64;

        let exp = now + 15 * 60; // 15 minutes
        let claims = Claims {
            sub:  staff_id.to_string(),
            role: role.to_string(),
            exp,
            iat:  now,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok((token, 900))
    }
}

impl JwtServiceImpl {
    pub fn decode_access(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|d| d.claims)
        .map_err(|e| AppError::Unauthorized(e.to_string()))
    }
}
