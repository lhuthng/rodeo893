use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use deadpool_redis::{Config as RedisConfig, Runtime};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    payment::{
        bank_transfer::BankTransferGateway,
        gateway_registry::PaymentGatewayRegistry,
    },
    config::AppConfig,
};
use api::{router::build_router, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env in dev
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info,bakery_backend=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env()
        .map_err(|e| anyhow::anyhow!("Config error: {}", e))?;

    // Database
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&db).await?;
    tracing::info!("Migrations applied");

    // Redis
    let redis = RedisConfig::from_url(&config.redis_url)
        .create_pool(Some(Runtime::Tokio1))?;

    // Payment gateway registry
    let mut gw_registry = PaymentGatewayRegistry::new();
    gw_registry.register("bank_transfer", Box::new(BankTransferGateway {
        account_number: std::env::var("BANK_ACCOUNT_NUMBER").unwrap_or_default(),
        account_name:   std::env::var("BANK_ACCOUNT_NAME").unwrap_or_default(),
        bank_name:      std::env::var("BANK_NAME").unwrap_or_default(),
    }));

    let state = AppState {
        db:    db.clone(),
        redis: redis.clone(),
        hasher:  Arc::new(Argon2Hasher),
        jwt:     Arc::new(JwtServiceImpl { secret: config.jwt_secret.clone() }),
        product_cache:  Arc::new(ProductCache::new(redis.clone())),
        tracking_cache: Arc::new(TrackingCache::new(redis.clone())),
        payment_gateway: Arc::new(gw_registry),
        user_repo:             Arc::new(PgUserRepository(db.clone())),
        session_repo:          Arc::new(PgUserSessionRepository(db.clone())),
        address_repo:          Arc::new(PgUserAddressRepository(db.clone())),
        staff_repo:            Arc::new(PgStaffRepository(db.clone())),
        refresh_token_repo:    Arc::new(PgStaffRefreshTokenRepository(db.clone())),
        membership_repo:       Arc::new(PgMembershipRepository(db.clone())),
        credit_repo:           Arc::new(PgCreditLedgerRepository(db.clone())),
        product_cat_repo:      Arc::new(PgProductCategoryRepository(db.clone())),
        product_repo:          Arc::new(PgProductRepository(db.clone())),
        ordering_day_repo:     Arc::new(PgProductOrderingDayRepository(db.clone())),
        stock_repo:            Arc::new(PgStockBatchRepository(db.clone())),
        occasion_repo:         Arc::new(PgOccasionRepository(db.clone())),
        occ_product_repo:      Arc::new(PgOccasionProductRepository(db.clone())),
        occ_announce_repo:     Arc::new(PgOccasionAnnouncementRepository(db.clone())),
        order_repo:            Arc::new(PgOrderRepository(db.clone())),
        order_item_repo:       Arc::new(PgOrderItemRepository(db.clone())),
        delivery_repo:         Arc::new(PgOrderDeliveryInfoRepository(db.clone())),
        payment_provider_repo: Arc::new(PgPaymentProviderRepository(db.clone())),
        payment_repo:          Arc::new(PgPaymentRepository(db.clone())),
        payment_event_repo:    Arc::new(PgPaymentEventRepository(db.clone())),
        refund_repo:           Arc::new(PgRefundRepository(db.clone())),
        tracking_repo:         Arc::new(PgTrackingRepository(db.clone())),
        notification_repo:     Arc::new(PgEmailNotificationRepository(db.clone())),
    };

    let router = build_router(state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, router).await?;

    Ok(())
}
