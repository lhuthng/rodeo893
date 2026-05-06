use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Job payload for sending a queued email notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailJob {
    pub notification_id: Uuid,
}
