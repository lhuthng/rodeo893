use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url:     String,
    pub redis_url:        String,
    pub session_secret:   String,
    pub jwt_secret:       String,
    pub app_env:          String,
    pub membership_topup: rust_decimal::Decimal,
    pub smtp_host:        String,
    pub smtp_port:        u16,
    pub smtp_user:        String,
    pub smtp_pass:        String,
    pub smtp_from:        String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        let get = |key: &str| -> Result<String, String> {
            env::var(key).map_err(|_| format!("Missing env var: {}", key))
        };
        let membership_topup = get("MEMBERSHIP_TOPUP_AMOUNT")?
            .parse::<rust_decimal::Decimal>()
            .map_err(|e| format!("MEMBERSHIP_TOPUP_AMOUNT parse error: {}", e))?;

        Ok(Self {
            database_url:     get("DATABASE_URL")?,
            redis_url:        get("REDIS_URL")?,
            session_secret:   get("SESSION_SECRET")?,
            jwt_secret:       get("JWT_SECRET")?,
            app_env:          env::var("APP_ENV").unwrap_or_else(|_| "dev".into()),
            membership_topup,
            smtp_host:        env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".into()),
            smtp_port:        env::var("SMTP_PORT").ok()
                              .and_then(|s| s.parse().ok()).unwrap_or(587),
            smtp_user:        env::var("SMTP_USER").unwrap_or_default(),
            smtp_pass:        env::var("SMTP_PASS").unwrap_or_default(),
            smtp_from:        env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@bakery.vn".into()),
        })
    }
}
