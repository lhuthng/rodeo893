use serde::{Deserialize, Serialize};

/// Periodic job to poll carrier APIs for stale tracking entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncTrackingJob {
    pub limit: i64,
}

impl Default for SyncTrackingJob {
    fn default() -> Self { Self { limit: 50 } }
}
