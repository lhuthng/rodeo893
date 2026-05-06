use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailNotification {
    pub id:              Uuid,
    pub recipient_email: String,
    pub template_name:   String,
    pub payload:         Value,
    pub status:          NotificationStatus,
    pub error_text:      Option<String>,
    pub scheduled_at:    DateTime<Utc>,
    pub sent_at:         Option<DateTime<Utc>>,
    pub created_at:      DateTime<Utc>,
}
