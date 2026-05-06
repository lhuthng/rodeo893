use async_trait::async_trait;
use application::error::AppError;
use super::delivery_registry::DeliveryProvider;

pub struct ViettelPostProvider { pub api_token: String }

#[async_trait]
impl DeliveryProvider for ViettelPostProvider {
    async fn get_tracking_status(&self, tracking_number: &str) -> Result<String, AppError> {
        Ok(format!("ViettelPost:{} pending integration", tracking_number))
    }
}
