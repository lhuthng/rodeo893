use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use domain::repositories::product::{ProductOrderingDayRepository, ProductRepository};
use crate::{dto::product::UpdateProductInput, error::AppError};

pub struct UpdateProduct<PR, ODR> {
    pub product_repo:  Arc<PR>,
    pub ordering_repo: Arc<ODR>,
}

impl<PR, ODR> UpdateProduct<PR, ODR>
where
    PR:  ProductRepository,
    ODR: ProductOrderingDayRepository,
{
    pub async fn execute(&self, id: Uuid, input: UpdateProductInput) -> Result<(), AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        let mut product = self.product_repo.find_by_id(id).await?;

        if let Some(name)        = input.name        { product.name        = name; }
        if let Some(description) = input.description { product.description = description; }
        if let Some(image_url)   = input.image_url   { product.image_url   = Some(image_url); }
        if let Some(base_price)  = input.base_price  { product.base_price  = base_price; }
        if let Some(is_active)   = input.is_active   { product.is_active   = is_active; }
        product.updated_at = Utc::now();

        self.product_repo.update(&product).await?;

        if let Some(days) = input.ordering_days {
            for &day in &days {
                if !(0..=6).contains(&day) {
                    return Err(AppError::Validation(format!("Invalid day_of_week: {}", day)));
                }
            }
            self.ordering_repo.set_for_product(id, &days).await?;
        }

        Ok(())
    }
}
