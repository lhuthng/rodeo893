use axum::{
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;
use application::error::AppError;
use crate::error::ApiError;

/// Extractor that deserializes JSON and validates it via the `Validate` trait.
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| ApiError(AppError::Validation(e.body_text())))?;
        value.validate().map_err(|e| ApiError(AppError::Validation(e.to_string())))?;
        Ok(Self(value))
    }
}
