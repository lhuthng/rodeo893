use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use domain::{
    entities::product::{Product, ProductCategory, ProductOrderingDay},
    repositories::product::{ProductCategoryRepository, ProductOrderingDayRepository, ProductRepository},
};
use crate::{dto::product::{CreateProductInput, ProductDto}, error::AppError};

pub struct CreateProduct<PR, CR, ODR> {
    pub product_repo:  Arc<PR>,
    pub category_repo: Arc<CR>,
    pub ordering_repo: Arc<ODR>,
}

impl<PR, CR, ODR> CreateProduct<PR, CR, ODR>
where
    PR:  ProductRepository,
    CR:  ProductCategoryRepository,
    ODR: ProductOrderingDayRepository,
{
    pub async fn execute(&self, input: CreateProductInput) -> Result<ProductDto, AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        let category = self.category_repo.find_by_id(input.category_id).await?;

        let now = Utc::now();
        let product = Product {
            id:           Uuid::new_v4(),
            category_id:  input.category_id,
            name:         input.name,
            slug:         input.slug,
            description:  input.description.unwrap_or_default(),
            image_url:    input.image_url,
            image_alt:    None,
            plating:      None,
            frontend_key: None,
            base_price:   input.base_price,
            currency:     "VND".into(),
            is_active:    true,
            created_at:   now,
            updated_at:   now,
        };

        let product = self.product_repo.create(&product).await?;

        // Validate days (0–6)
        for &day in &input.ordering_days {
            if !(0..=6).contains(&day) {
                return Err(AppError::Validation(format!("Invalid day_of_week: {}", day)));
            }
        }
        self.ordering_repo.set_for_product(product.id, &input.ordering_days).await?;

        Ok(ProductDto {
            id:            product.id,
            category_id:   product.category_id,
            category_slug: category.slug,
            name:          product.name,
            slug:          product.slug,
            description:   product.description,
            image_url:     product.image_url,
            base_price:    product.base_price,
            currency:      product.currency,
            ordering_days: input.ordering_days,
            is_active:     product.is_active,
        })
    }
}
