use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserInput {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 9, max = 15, message = "Phone must be 9–15 characters"))]
    pub phone: String,

    #[validate(length(min = 1, max = 100, message = "Display name required"))]
    pub display_name: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token:        String,
    pub expires_at:   chrono::DateTime<chrono::Utc>,
    pub user:         super::user::UserDto,
}
