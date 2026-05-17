use deadpool_redis::Pool;
use redis::AsyncCommands;
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
        conn.get::<_, Option<String>>(key).await.ok().flatten()
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        conn.set_ex::<_, _, ()>(key, value, self.ttl).await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }

    pub async fn invalidate(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        conn.del::<_, ()>(key).await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(())
    }
}
