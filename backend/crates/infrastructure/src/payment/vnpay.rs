use application::{error::AppError, use_cases::activate_membership::{PaymentCreationResult, PaymentGateway}};
use async_trait::async_trait;
use rust_decimal::Decimal;

pub struct VnPayGateway {
    pub terminal_id: String,
    pub secret_key:  String,
    pub api_url:     String,
}

#[async_trait]
impl PaymentGateway for VnPayGateway {
    async fn create_payment(
        &self,
        _provider_code: &str,
        amount: Decimal,
        _currency: &str,
        _purpose: &str,
        reference: &str,
    ) -> Result<PaymentCreationResult, AppError> {
        let gateway_ref = format!("VP-{}", reference);
        let instructions = Some(format!(
            "Redirect to VnPay payment URL for {} (ref: {})", amount, gateway_ref
        ));
        Ok(PaymentCreationResult { gateway_ref, instructions })
    }
}
