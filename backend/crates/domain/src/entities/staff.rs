use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "staff_role", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum StaffRole {
    Owner,
    Manager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id:            Uuid,
    pub name:          String,
    pub email:         String,
    pub password_hash: String,
    pub role:          StaffRole,
    pub is_active:     bool,
    pub created_at:    DateTime<Utc>,
    pub updated_at:    DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffRefreshToken {
    pub id:         Uuid,
    pub staff_id:   Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
