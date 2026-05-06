use axum::{
    async_trait,
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;
use application::error::AppError;

/// Extractor that deserializes JSON and validates it via the `Validate` trait.
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::Validation(e.body_text()))?;
        value.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        Ok(Self(value))
    }
}
