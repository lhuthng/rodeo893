use application::{error::AppError, use_cases::activate_membership::{PaymentCreationResult, PaymentGateway}};
use async_trait::async_trait;
use rust_decimal::Decimal;

/// Manual bank transfer — provides instructions with account details.
pub struct BankTransferGateway {
    pub account_number: String,
    pub account_name:   String,
    pub bank_name:      String,
}

#[async_trait]
impl PaymentGateway for BankTransferGateway {
    async fn create_payment(
        &self,
        _provider_code: &str,
        amount: Decimal,
        currency: &str,
        _purpose: &str,
        reference: &str,
    ) -> Result<PaymentCreationResult, AppError> {
        let instructions = format!(
            "Transfer {} {} to:\nBank: {}\nAccount: {}\nName: {}\nContent: {}",
            amount, currency, self.bank_name, self.account_number, self.account_name, reference
        );
        Ok(PaymentCreationResult {
              gateway_ref:  Some(reference.to_string()),
              instructions: serde_json::json!({ "text": instructions }),
        })
    }
}
