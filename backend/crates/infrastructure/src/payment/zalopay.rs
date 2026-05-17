use application::{error::AppError, use_cases::activate_membership::{PaymentCreationResult, PaymentGateway}};
use async_trait::async_trait;
use rust_decimal::Decimal;

pub struct ZaloPayGateway {
    pub app_id:  String,
    pub key1:    String,
    pub key2:    String,
    pub api_url: String,
}

#[async_trait]
impl PaymentGateway for ZaloPayGateway {
    async fn create_payment(
        &self,
        _provider_code: &str,
        amount: Decimal,
        _currency: &str,
        _purpose: &str,
        reference: &str,
    ) -> Result<PaymentCreationResult, AppError> {
        // Stub: real integration would POST to ZaloPay /v2/create
        let gateway_ref = format!("ZP-{}", reference);
        let instructions = Some(format!(
            "Redirect to ZaloPay payment URL for {} (ref: {})",
            amount, gateway_ref
        ));
        Ok(PaymentCreationResult {
            gateway_ref: Some(gateway_ref),
            instructions: serde_json::json!({ "text": instructions }),
        })
    }
}
