use std::sync::Arc;
use uuid::Uuid;
use domain::repositories::product::ProductRepository;
use crate::error::AppError;

pub struct DeactivateProduct<PR> {
    pub product_repo: Arc<PR>,
}

impl<PR: ProductRepository> DeactivateProduct<PR> {
    pub async fn execute(&self, id: Uuid) -> Result<(), AppError> {
        let has_open = self.product_repo.has_open_orders(id).await?;
        if has_open {
            return Err(AppError::Conflict(
                "Cannot deactivate product with open orders".into()
            ));
        }
        self.product_repo.deactivate(id).await?;
        Ok(())
    }
}
