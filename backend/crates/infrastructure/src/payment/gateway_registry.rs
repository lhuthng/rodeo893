use std::collections::HashMap;
use application::{error::AppError, use_cases::activate_membership::{PaymentCreationResult, PaymentGateway}};
use async_trait::async_trait;

/// Registry maps provider codes to gateway implementations.
pub struct PaymentGatewayRegistry {
    gateways: HashMap<String, Box<dyn PaymentGateway>>,
}

impl PaymentGatewayRegistry {
    pub fn new() -> Self {
        Self { gateways: HashMap::new() }
    }

    pub fn register(&mut self, code: impl Into<String>, gw: Box<dyn PaymentGateway>) {
        self.gateways.insert(code.into(), gw);
    }
}

#[async_trait]
impl PaymentGateway for PaymentGatewayRegistry {
    async fn create_payment(
        &self,
        provider_code: &str,
        amount: rust_decimal::Decimal,
        currency: &str,
        purpose: &str,
        reference: &str,
    ) -> Result<PaymentCreationResult, AppError> {
        let gw = self.gateways.get(provider_code)
            .ok_or_else(|| AppError::Validation(format!("Unknown provider: {}", provider_code)))?;
        gw.create_payment(provider_code, amount, currency, purpose, reference).await
    }
}
