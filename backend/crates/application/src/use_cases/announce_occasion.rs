use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

use domain::{
    entities::occasion::OccasionAnnouncement,
    entities::notification::{EmailNotification, NotificationStatus},
    repositories::{
        occasion::{OccasionAnnouncementRepository, OccasionRepository},
        notification::EmailNotificationRepository,
        user::UserRepository,
    },
};
use crate::error::AppError;

pub struct AnnounceOccasion<OR, OAR, NR, UR> {
    pub occasion_repo:      Arc<OR>,
    pub announcement_repo:  Arc<OAR>,
    pub notification_repo:  Arc<NR>,
    pub user_repo:          Arc<UR>,
}

impl<OR, OAR, NR, UR> AnnounceOccasion<OR, OAR, NR, UR>
where
    OR:  OccasionRepository,
    OAR: OccasionAnnouncementRepository,
    NR:  EmailNotificationRepository,
    UR:  UserRepository,
{
    /// Queue announcement emails to all active members.
    /// Returns the number of emails queued.
    pub async fn execute(&self, occasion_id: Uuid) -> Result<i32, AppError> {
        let occasion = self.occasion_repo.find_by_id(occasion_id).await?;
        if !occasion.is_active {
            return Err(AppError::Validation("Occasion is not active".into()));
        }

        // List all active users — in practice should filter to members with active membership;
        // infrastructure layer handles the membership join
        let users = self.user_repo.list(10_000, 0).await?;
        let now = Utc::now();
        let mut count = 0i32;

        for user in &users {
            let notification = EmailNotification {
                id:              Uuid::new_v4(),
                recipient_email: user.email.clone(),
                template_name:   "occasion_announcement".into(),
                payload:         serde_json::json!({
                    "occasion_id":    occasion.id,
                    "title":          occasion.title,
                    "order_open_at":  occasion.order_open_at,
                    "order_close_at": occasion.order_close_at,
                }),
                status:          NotificationStatus::Pending,
                error_text:      None,
                scheduled_at:    now,
                sent_at:         None,
                created_at:      now,
            };
            self.notification_repo.create(&notification).await?;
            count += 1;
        }

        let announcement = OccasionAnnouncement {
            id:              Uuid::new_v4(),
            occasion_id,
            sent_at:         now,
            recipient_count: count,
            status:          "queued".into(),
        };
        self.announcement_repo.create(&announcement).await?;

        Ok(count)
    }
}
