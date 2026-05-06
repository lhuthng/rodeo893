use thiserror::Error;
use domain::DomainError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("Validation: {0}")]
    Validation(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Payment error: {0}")]
    Payment(String),

    #[error("Internal error: {0}")]
    Internal(String),
}
