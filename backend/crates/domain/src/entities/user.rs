use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id:            Uuid,
    pub email:         String,
    pub phone:         String,
    pub display_name:  String,
    pub password_hash: String,
    pub is_active:     bool,
    pub created_at:    DateTime<Utc>,
    pub updated_at:    DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id:         Uuid,
    pub user_id:    Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub ip_addr:    Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAddress {
    pub id:             Uuid,
    pub user_id:        Uuid,
    pub label:          String,
    pub recipient_name: String,
    pub phone:          String,
    pub street:         String,
    pub ward:           String,
    pub district:       String,
    pub city:           String,
    pub country:        String,
    pub is_default:     bool,
    pub created_at:     DateTime<Utc>,
}
