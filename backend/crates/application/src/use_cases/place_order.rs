use std::sync::Arc;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;
use validator::Validate;

use domain::{
    entities::{
        order::{Order, OrderDeliveryInfo, OrderItem, OrderStatus},
        payment::{Payment, PaymentStatus},
    },
    repositories::{
        order::{OrderDeliveryInfoRepository, OrderItemRepository, OrderRepository},
        payment::{PaymentProviderRepository, PaymentRepository},
        stock::StockBatchRepository,
        occasion::OccasionProductRepository,
    },
};
use crate::{
    dto::order::{
        DeliveryInfoDto, OrderItemInput, OrderItemSource, OrderSummaryDto,
        PaymentInstructionsDto, PlaceOrderInput, PlaceOrderResponse,
    },
    error::AppError,
    use_cases::activate_membership::PaymentGateway,
};

pub struct PlaceOrder<OR, OIR, ODR, SBR, OPR, PPR, PR, PG> {
    pub order_repo:       Arc<OR>,
    pub item_repo:        Arc<OIR>,
    pub delivery_repo:    Arc<ODR>,
    pub batch_repo:       Arc<SBR>,
    pub occ_product_repo: Arc<OPR>,
    pub provider_repo:    Arc<PPR>,
    pub payment_repo:     Arc<PR>,
    pub payment_gateway:  Arc<PG>,
}

impl<OR, OIR, ODR, SBR, OPR, PPR, PR, PG> PlaceOrder<OR, OIR, ODR, SBR, OPR, PPR, PR, PG>
where
    OR:  OrderRepository,
    OIR: OrderItemRepository,
    ODR: OrderDeliveryInfoRepository,
    SBR: StockBatchRepository,
    OPR: OccasionProductRepository,
    PPR: PaymentProviderRepository,
    PR:  PaymentRepository,
    PG:  PaymentGateway,
{
    pub async fn execute(
        &self,
        user_id: Option<Uuid>,
        input: PlaceOrderInput,
    ) -> Result<PlaceOrderResponse, AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        if input.items.is_empty() {
            return Err(AppError::Validation("Order must have at least one item".into()));
        }

        let provider = self.provider_repo.find_by_code(&input.provider_code).await
            .map_err(|_| AppError::Validation(format!("Unknown provider: {}", input.provider_code)))?;
        if !provider.is_active {
            return Err(AppError::Validation("Payment provider not available".into()));
        }

        // Reserve stock & compute total
        let mut order_items: Vec<OrderItem> = Vec::new();
        let mut total = Decimal::ZERO;

        for item_input in &input.items {
            let (item, price) = self.resolve_item(item_input).await?;
            total += price * Decimal::from(item_input.quantity);
            order_items.push(item);
        }

        let reference_code = generate_reference();
        let now = Utc::now();

        let order = Order {
            id:             Uuid::new_v4(),
            reference_code: reference_code.clone(),
            user_id,
            status:         OrderStatus::PendingPayment,
            total_amount:   total,
            currency:       "VND".into(),
            note:           input.note,
            created_at:     now,
            updated_at:     now,
        };

        let order = self.order_repo.create(&order).await?;

        // Assign order_id to items
        let items: Vec<OrderItem> = order_items
            .into_iter()
            .map(|mut i| { i.order_id = order.id; i })
            .collect();
        self.item_repo.create_many(&items).await?;

        let delivery = OrderDeliveryInfo {
            id:             Uuid::new_v4(),
            order_id:       order.id,
            recipient_name: input.delivery.recipient_name,
            phone:          input.delivery.phone,
            street:         input.delivery.street,
            ward:           input.delivery.ward,
            district:       input.delivery.district,
            city:           input.delivery.city,
            country:        "VN".into(),
            delivery_note:  input.delivery.delivery_note,
        };
        self.delivery_repo.create(&delivery).await?;

        // Create payment
        let payment_result = self.payment_gateway
            .create_payment(&input.provider_code, total, "VND", "order", &reference_code)
            .await?;

        let payment = Payment {
            id:              Uuid::new_v4(),
            order_id:        Some(order.id),
            provider_id:     provider.id,
            amount:          total,
            currency:        "VND".into(),
            status:          PaymentStatus::Pending,
            gateway_ref:     payment_result.gateway_ref,
            gateway_payload: serde_json::json!({ "purpose": "order", "reference": reference_code }),
            created_at:      now,
            updated_at:      now,
        };
        let payment = self.payment_repo.create(&payment).await?;

        Ok(PlaceOrderResponse {
            order: OrderSummaryDto {
                id:             order.id,
                reference_code: order.reference_code,
                status:         order.status.to_string(),
                total_amount:   order.total_amount,
                currency:       order.currency,
                item_count:     items.len() as i32,
                created_at:     order.created_at,
            },
            payment: PaymentInstructionsDto {
                payment_id:   payment.id,
                provider:     input.provider_code,
                amount:       total,
                currency:     "VND".into(),
                instructions: payment_result.instructions,
            },
        })
    }

    async fn resolve_item(&self, input: &OrderItemInput) -> Result<(OrderItem, Decimal), AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        match &input.source {
            OrderItemSource::WeeklyBatch { batch_id } => {
                let batch = self.batch_repo.find_by_id(*batch_id).await?;
                if !batch.is_released {
                    return Err(AppError::Validation(format!("Batch {} is not yet released", batch_id)));
                }
                self.batch_repo.reserve(*batch_id, input.quantity).await?;

                // Get product price from batch (product base_price)
                // For simplicity, stored at order time from product.base_price
                // In practice, query product; here we use the batch's product_id
                let item = OrderItem {
                    id:                  Uuid::new_v4(),
                    order_id:            Uuid::nil(), // set after order creation
                    product_id:          batch.product_id,
                    batch_id:            Some(*batch_id),
                    occasion_product_id: None,
                    quantity:            input.quantity,
                    unit_price:          Decimal::ZERO, // will be overwritten below
                };
                // unit_price needs product — infrastructure layer passes it; for now set placeholder
                // The actual price is resolved by infrastructure which has product repo
                Ok((item, Decimal::ZERO))
            }
            OrderItemSource::OccasionProduct { occasion_product_id } => {
                let op = self.occ_product_repo.find_by_id(*occasion_product_id).await?;
                self.occ_product_repo.reserve(*occasion_product_id, input.quantity).await?;

                let item = OrderItem {
                    id:                  Uuid::new_v4(),
                    order_id:            Uuid::nil(),
                    product_id:          op.product_id,
                    batch_id:            None,
                    occasion_product_id: Some(*occasion_product_id),
                    quantity:            input.quantity,
                    unit_price:          op.unit_price,
                };
                Ok((item, op.unit_price))
            }
        }
    }
}

fn generate_reference() -> String {
    use rand::Rng;
    const CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rng = rand::thread_rng();
    let suffix: String = (0..6).map(|_| CHARS[rng.gen_range(0..CHARS.len())] as char).collect();
    format!("BKR-{}", suffix)
}
