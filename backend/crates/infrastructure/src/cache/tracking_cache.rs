use deadpool_redis::Pool;
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
        let val: Option<String> = redis::cmd("GET")
            .arg(format!("tracking:{}", tracking_number))
            .query_async(&mut conn)
            .await
            .ok()?;
        val
    }

    pub async fn set(&self, tracking_number: &str, status: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        redis::cmd("SETEX")
            .arg(format!("tracking:{}", tracking_number))
            .arg(self.ttl)
            .arg(status)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }
}
