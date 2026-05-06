use std::sync::Arc;
use uuid::Uuid;
use domain::repositories::membership::MembershipRepository;
use crate::{dto::membership::MembershipDto, error::AppError};

pub struct GetMembershipStatus<MR> {
    pub membership_repo: Arc<MR>,
}

impl<MR: MembershipRepository> GetMembershipStatus<MR> {
    pub async fn execute(&self, user_id: Uuid) -> Result<Option<MembershipDto>, AppError> {
        let membership = self.membership_repo.find_by_user_id(user_id).await?;
        Ok(membership.map(|m| MembershipDto {
            id:           m.id,
            is_active:    m.is_currently_active(),
            activated_at: m.activated_at,
            expires_at:   m.expires_at,
        }))
    }
}
