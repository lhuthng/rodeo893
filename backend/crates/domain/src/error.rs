use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Insufficient stock for batch {batch_id}: requested {requested}, available {available}")]
    InsufficientStock {
        batch_id:  uuid::Uuid,
        requested: i32,
        available: i32,
    },

    #[error("Order {reference} cannot be cancelled in status {status}")]
    OrderNotCancellable { reference: String, status: String },

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal error: {0}")]
    Internal(String),
}
