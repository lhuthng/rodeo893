use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use application::error::AppError;

#[derive(Debug)]
pub struct ApiError(pub AppError);

pub type ApiResult<T> = Result<T, ApiError>;

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        Self(value)
    }
}

impl From<domain::DomainError> for ApiError {
    fn from(value: domain::DomainError) -> Self {
        Self(AppError::from(value))
    }
}

/// Converts application errors into an Axum JSON response.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self.0 {
            AppError::Validation(m) => (StatusCode::UNPROCESSABLE_ENTITY, "validation_error", m.as_str()),
            AppError::Unauthorized(m) => (StatusCode::UNAUTHORIZED, "unauthorized", m.as_str()),
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, "not_found", m.as_str()),
            AppError::Conflict(m) => (StatusCode::CONFLICT, "conflict", m.as_str()),
            AppError::Payment(m) => (StatusCode::BAD_GATEWAY, "payment_error", m.as_str()),
            AppError::Domain(d) => {
                use domain::DomainError;
                match d {
                    DomainError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found", "Resource not found"),
                    DomainError::Conflict(_) => (StatusCode::CONFLICT, "conflict", "Conflict"),
                    DomainError::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "validation_error", "Validation failed"),
                    DomainError::InsufficientStock { .. } => (StatusCode::CONFLICT, "insufficient_stock", "Insufficient stock"),
                    DomainError::OrderNotCancellable { .. } => (StatusCode::CONFLICT, "order_not_cancellable", "Order cannot be cancelled"),
                    DomainError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", "Unauthorized"),
                    DomainError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", "Internal server error"),
                }
            }
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", "Internal server error"),
        };

        let body = Json(json!({ "error": code, "message": message }));
        (status, body).into_response()
    }
}
