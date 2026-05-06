use std::sync::Arc;
use uuid::Uuid;
use domain::repositories::{
    user::{UserAddressRepository, UserRepository},
    membership::{CreditLedgerRepository, MembershipRepository},
};
use crate::{
    dto::user::{AddressDto, MembershipStatusDto, UserProfileDto},
    error::AppError,
};

pub struct GetUserProfile<UR, AR, MR, CR> {
    pub user_repo:       Arc<UR>,
    pub address_repo:    Arc<AR>,
    pub membership_repo: Arc<MR>,
    pub credit_repo:     Arc<CR>,
}

impl<UR, AR, MR, CR> GetUserProfile<UR, AR, MR, CR>
where
    UR: UserRepository,
    AR: UserAddressRepository,
    MR: MembershipRepository,
    CR: CreditLedgerRepository,
{
    pub async fn execute(&self, user_id: Uuid) -> Result<UserProfileDto, AppError> {
        let user = self.user_repo.find_by_id(user_id).await?;
        let addresses = self.address_repo.list_for_user(user_id).await?;
        let membership = self.membership_repo.find_by_user_id(user_id).await?;
        let credit_balance = self.credit_repo.balance_for_user(user_id).await?;

        let membership_dto = membership.map(|m| MembershipStatusDto {
            is_active:    m.is_currently_active(),
            activated_at: m.activated_at,
            expires_at:   m.expires_at,
        });

        let address_dtos = addresses
            .into_iter()
            .map(|a| AddressDto {
                id:             a.id,
                label:          a.label,
                recipient_name: a.recipient_name,
                phone:          a.phone,
                street:         a.street,
                ward:           a.ward,
                district:       a.district,
                city:           a.city,
                country:        a.country,
                is_default:     a.is_default,
            })
            .collect();

        Ok(UserProfileDto {
            id:             user.id,
            email:          user.email,
            phone:          user.phone,
            display_name:   user.display_name,
            is_active:      user.is_active,
            created_at:     user.created_at,
            membership:     membership_dto,
            credit_balance,
            addresses:      address_dtos,
        })
    }
}
