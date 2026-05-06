use std::sync::Arc;
use chrono::Utc;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use uuid::Uuid;
use validator::Validate;

use domain::{
    entities::membership::{CreditEntryType, CreditLedgerEntry, Membership},
    repositories::{
        membership::{CreditLedgerRepository, MembershipRepository},
        payment::{PaymentProviderRepository, PaymentRepository},
    },
};
use crate::{
    dto::membership::{ActivateMembershipInput, ActivateMembershipResponse},
    error::AppError,
};

/// Trait for the payment gateway — infrastructure implements this.
#[async_trait::async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn create_payment(
        &self,
        provider_code: &str,
        amount: Decimal,
        currency: &str,
        purpose: &str,
        reference: &str,
    ) -> Result<PaymentCreationResult, AppError>;
}

pub struct PaymentCreationResult {
    pub gateway_ref:  Option<String>,
    pub instructions: serde_json::Value,
}

pub struct ActivateMembership<MR, CR, PPR, PR, PG> {
    pub membership_repo:  Arc<MR>,
    pub credit_repo:      Arc<CR>,
    pub provider_repo:    Arc<PPR>,
    pub payment_repo:     Arc<PR>,
    pub payment_gateway:  Arc<PG>,
    pub topup_amount_vnd: Decimal,
}

impl<MR, CR, PPR, PR, PG> ActivateMembership<MR, CR, PPR, PR, PG>
where
    MR:  MembershipRepository,
    CR:  CreditLedgerRepository,
    PPR: PaymentProviderRepository,
    PR:  PaymentRepository,
    PG:  PaymentGateway,
{
    pub async fn execute(
        &self,
        user_id: Uuid,
        input: ActivateMembershipInput,
    ) -> Result<ActivateMembershipResponse, AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        // Idempotency: if already an active member, reject
        if let Some(m) = self.membership_repo.find_by_user_id(user_id).await? {
            if m.is_currently_active() {
                return Err(AppError::Conflict("User is already an active member".into()));
            }
        }

        let provider = self.provider_repo.find_by_code(&input.provider_code).await
            .map_err(|_| AppError::Validation(format!("Unknown provider: {}", input.provider_code)))?;
        if !provider.is_active {
            return Err(AppError::Validation(format!("Payment provider {} is not available", input.provider_code)));
        }

        let reference = format!("MEMBER-{}", Uuid::new_v4().to_string()[..8].to_uppercase());

        let result = self.payment_gateway
            .create_payment(&input.provider_code, self.topup_amount_vnd, "VND", "membership", &reference)
            .await?;

        // Create payment record
        let payment = domain::entities::payment::Payment {
            id:              Uuid::new_v4(),
            order_id:        None,
            provider_id:     provider.id,
            amount:          self.topup_amount_vnd,
            currency:        "VND".into(),
            status:          domain::entities::payment::PaymentStatus::Pending,
            gateway_ref:     result.gateway_ref,
            gateway_payload: serde_json::json!({ "purpose": "membership", "user_id": user_id, "reference": reference }),
            created_at:      Utc::now(),
            updated_at:      Utc::now(),
        };
        let payment = self.payment_repo.create(&payment).await?;

        Ok(ActivateMembershipResponse {
            payment_id:   payment.id,
            amount:       self.topup_amount_vnd,
            currency:     "VND".into(),
            instructions: result.instructions,
        })
    }

    /// Called after payment gateway confirms payment completed.
    pub async fn confirm(
        &self,
        user_id: Uuid,
        payment_id: Uuid,
    ) -> Result<(), AppError> {
        let payment = self.payment_repo.find_by_id(payment_id).await?;
        if payment.status != domain::entities::payment::PaymentStatus::Completed {
            return Err(AppError::Payment("Payment not completed".into()));
        }

        let now = Utc::now();
        let membership = Membership {
            id:                      Uuid::new_v4(),
            user_id,
            activated_at:            now,
            expires_at:              None, // lifetime
            activated_by_payment_id: Some(payment_id),
            is_active:               true,
            created_at:              now,
        };

        // Upsert: if prior inactive membership, update; else create
        match self.membership_repo.find_by_user_id(user_id).await? {
            Some(mut existing) => {
                existing.activated_at            = now;
                existing.is_active               = true;
                existing.activated_by_payment_id = Some(payment_id);
                self.membership_repo.update(&existing).await?;
            }
            None => {
                self.membership_repo.create(&membership).await?;
            }
        }

        // Record top-up in credit ledger
        let entry = CreditLedgerEntry {
            id:          Uuid::new_v4(),
            user_id,
            order_id:    None,
            amount:      payment.amount,
            entry_type:  CreditEntryType::MembershipTopup,
            description: "Membership activation top-up".into(),
            created_at:  now,
        };
        self.credit_repo.append(&entry).await?;

        Ok(())
    }
}
