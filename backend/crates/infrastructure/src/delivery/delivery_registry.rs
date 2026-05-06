use async_trait::async_trait;
use application::error::AppError;

#[async_trait]
pub trait DeliveryProvider: Send + Sync {
    async fn get_tracking_status(&self, tracking_number: &str) -> Result<String, AppError>;
}

pub struct DeliveryRegistry {
    providers: std::collections::HashMap<String, Box<dyn DeliveryProvider>>,
}

impl DeliveryRegistry {
    pub fn new() -> Self {
        Self { providers: std::collections::HashMap::new() }
    }

    pub fn register(&mut self, code: impl Into<String>, p: Box<dyn DeliveryProvider>) {
        self.providers.insert(code.into(), p);
    }

    pub async fn get_status(&self, carrier: &str, tracking_number: &str) -> Result<String, AppError> {
        let p = self.providers.get(carrier)
            .ok_or_else(|| AppError::Validation(format!("Unknown carrier: {}", carrier)))?;
        p.get_tracking_status(tracking_number).await
    }
}
