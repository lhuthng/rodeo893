use deadpool_redis::Pool;
use application::error::AppError;

pub struct ProductCache {
    pool: Pool,
    ttl:  u64,
}

impl ProductCache {
    pub fn new(pool: Pool) -> Self {
        Self { pool, ttl: 300 } // 5 minutes
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let mut conn = self.pool.get().await.ok()?;
        let val: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut conn)
            .await
            .ok()?;
        val
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        redis::cmd("SETEX")
            .arg(key)
            .arg(self.ttl)
            .arg(value)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }

    pub async fn invalidate(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        redis::cmd("DEL")
            .arg(key)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }
}
