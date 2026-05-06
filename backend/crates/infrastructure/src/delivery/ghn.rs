use async_trait::async_trait;
use application::error::AppError;
use super::delivery_registry::DeliveryProvider;

pub struct GhnProvider { pub api_token: String, pub shop_id: String }

#[async_trait]
impl DeliveryProvider for GhnProvider {
    async fn get_tracking_status(&self, tracking_number: &str) -> Result<String, AppError> {
        // TODO: call GHN API https://online-gateway.ghn.vn/shiip/public-api/v2/shipping-order/detail
        Ok(format!("GHN:{} pending integration", tracking_number))
    }
}
