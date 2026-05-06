use std::sync::Arc;
use uuid::Uuid;
use domain::repositories::user::UserSessionRepository;
use crate::error::AppError;

pub struct LogoutUser<SR> {
    pub session_repo: Arc<SR>,
}

impl<SR: UserSessionRepository> LogoutUser<SR> {
    pub async fn execute(&self, session_id: Uuid) -> Result<(), AppError> {
        self.session_repo.delete(session_id).await?;
        Ok(())
    }
}
