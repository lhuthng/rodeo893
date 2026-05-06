use std::sync::Arc;
use domain::repositories::{
    order::{OrderDeliveryInfoRepository, OrderItemRepository, OrderRepository},
    payment::PaymentRepository,
    tracking::TrackingRepository,
    product::ProductRepository,
};
use crate::{
    dto::order::{
        DeliveryInfoDto, OrderDetailDto, OrderItemDto, OrderSummaryDto,
        PaymentSummaryDto, TrackingDto,
    },
    error::AppError,
};

pub struct GetOrderByReference<OR, OIR, ODR, PR, TR, PRd> {
    pub order_repo:    Arc<OR>,
    pub item_repo:     Arc<OIR>,
    pub delivery_repo: Arc<ODR>,
    pub payment_repo:  Arc<PR>,
    pub tracking_repo: Arc<TR>,
    pub product_repo:  Arc<PRd>,
}

impl<OR, OIR, ODR, PR, TR, PRd> GetOrderByReference<OR, OIR, ODR, PR, TR, PRd>
where
    OR:  OrderRepository,
    OIR: OrderItemRepository,
    ODR: OrderDeliveryInfoRepository,
    PR:  PaymentRepository,
    TR:  TrackingRepository,
    PRd: ProductRepository,
{
    pub async fn execute(&self, reference: &str) -> Result<OrderDetailDto, AppError> {
        let order = self.order_repo.find_by_reference(reference).await?;
        let items = self.item_repo.list_for_order(order.id).await?;
        let delivery = self.delivery_repo.find_by_order(order.id).await?;

        let payment = self.payment_repo.find_by_order(order.id).await.ok();
        let tracking = self.tracking_repo.find_by_order(order.id).await?;

        let mut item_dtos = Vec::new();
        for item in items {
            let product = self.product_repo.find_by_id(item.product_id).await?;
            let subtotal = item.unit_price * rust_decimal::Decimal::from(item.quantity);
            item_dtos.push(OrderItemDto {
                product_id:   item.product_id,
                product_name: product.name,
                quantity:     item.quantity,
                unit_price:   item.unit_price,
                subtotal,
            });
        }

        let payment_dto = payment.map(|p| {
            // provider name lookup omitted here; infrastructure can enrich
            PaymentSummaryDto {
                provider:    p.provider_id.to_string(),
                amount:      p.amount,
                currency:    p.currency,
                status:      p.status.to_string(),
                gateway_ref: p.gateway_ref,
            }
        });

        let tracking_dto = tracking.map(|t| TrackingDto {
            carrier_code:    t.carrier_code,
            tracking_number: t.tracking_number,
            tracking_url:    t.tracking_url,
            last_status:     t.last_status,
        });

        Ok(OrderDetailDto {
            id:             order.id,
            reference_code: order.reference_code,
            status:         order.status.to_string(),
            total_amount:   order.total_amount,
            currency:       order.currency,
            note:           order.note,
            items:          item_dtos,
            delivery: DeliveryInfoDto {
                recipient_name: delivery.recipient_name,
                phone:          delivery.phone,
                street:         delivery.street,
                ward:           delivery.ward,
                district:       delivery.district,
                city:           delivery.city,
                country:        delivery.country,
                delivery_note:  delivery.delivery_note,
            },
            payment:  payment_dto,
            tracking: tracking_dto,
            created_at: order.created_at,
            updated_at: order.updated_at,
        })
    }
}
