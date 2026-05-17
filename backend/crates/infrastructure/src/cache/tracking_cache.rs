use deadpool_redis::Pool;
use redis::AsyncCommands;
use application::error::AppError;

pub struct TrackingCache {
    pool: Pool,
    ttl:  u64,
}

impl TrackingCache {
    pub fn new(pool: Pool) -> Self {
        Self { pool, ttl: 120 } // 2 minutes
    }

    pub async fn get(&self, tracking_number: &str) -> Option<String> {
        let mut conn = self.pool.get().await.ok()?;
        conn.get::<_, Option<String>>(format!("tracking:{}", tracking_number))
            .await.ok().flatten()
    }

    pub async fn set(&self, tracking_number: &str, status: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        conn.set_ex::<_, _, ()>(format!("tracking:{}", tracking_number), status, self.ttl)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }
}
