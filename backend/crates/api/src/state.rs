use std::sync::Arc;
use infrastructure::{
    auth::{argon2_hasher::Argon2Hasher, jwt_service::JwtServiceImpl},
    cache::{ProductCache, TrackingCache},
    database::{
        pg_membership_repository::{PgCreditLedgerRepository, PgMembershipRepository},
        pg_notification_repository::PgEmailNotificationRepository,
        pg_occasion_repository::{PgOccasionAnnouncementRepository, PgOccasionProductRepository, PgOccasionRepository},
        pg_order_repository::{PgOrderDeliveryInfoRepository, PgOrderItemRepository, PgOrderRepository},
        pg_payment_repository::{PgPaymentEventRepository, PgPaymentProviderRepository, PgPaymentRepository},
        pg_product_repository::{PgProductCategoryRepository, PgProductOrderingDayRepository, PgProductRepository},
        pg_refund_repository::PgRefundRepository,
        pg_staff_repository::{PgStaffRefreshTokenRepository, PgStaffRepository},
        pg_stock_repository::PgStockBatchRepository,
        pg_tracking_repository::PgTrackingRepository,
        pg_user_repository::{PgUserAddressRepository, PgUserRepository, PgUserSessionRepository},
    },
    payment::PaymentGatewayRegistry,
};
use sqlx::PgPool;
use deadpool_redis::Pool as RedisPool;
use rust_decimal::Decimal;

#[derive(Clone)]
pub struct AppState {
    pub db:    PgPool,
    pub redis: RedisPool,

    // Auth helpers
    pub hasher:  Arc<Argon2Hasher>,
    pub jwt:     Arc<JwtServiceImpl>,

    // Caches
    pub product_cache:  Arc<ProductCache>,
    pub tracking_cache: Arc<TrackingCache>,

    // Payment
    pub payment_gateway: Arc<PaymentGatewayRegistry>,
    pub membership_topup_amount: Decimal,

    // Repos (arc-wrapped so cloning AppState is cheap)
    pub user_repo:           Arc<PgUserRepository>,
    pub session_repo:        Arc<PgUserSessionRepository>,
    pub address_repo:        Arc<PgUserAddressRepository>,
    pub staff_repo:          Arc<PgStaffRepository>,
    pub refresh_token_repo:  Arc<PgStaffRefreshTokenRepository>,
    pub membership_repo:     Arc<PgMembershipRepository>,
    pub credit_repo:         Arc<PgCreditLedgerRepository>,
    pub product_cat_repo:    Arc<PgProductCategoryRepository>,
    pub product_repo:        Arc<PgProductRepository>,
    pub ordering_day_repo:   Arc<PgProductOrderingDayRepository>,
    pub stock_repo:          Arc<PgStockBatchRepository>,
    pub occasion_repo:       Arc<PgOccasionRepository>,
    pub occ_product_repo:    Arc<PgOccasionProductRepository>,
    pub occ_announce_repo:   Arc<PgOccasionAnnouncementRepository>,
    pub order_repo:          Arc<PgOrderRepository>,
    pub order_item_repo:     Arc<PgOrderItemRepository>,
    pub delivery_repo:       Arc<PgOrderDeliveryInfoRepository>,
    pub payment_provider_repo: Arc<PgPaymentProviderRepository>,
    pub payment_repo:        Arc<PgPaymentRepository>,
    pub payment_event_repo:  Arc<PgPaymentEventRepository>,
    pub refund_repo:         Arc<PgRefundRepository>,
    pub tracking_repo:       Arc<PgTrackingRepository>,
    pub notification_repo:   Arc<PgEmailNotificationRepository>,
}
