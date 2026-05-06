use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

use domain::{
    entities::user::UserAddress,
    repositories::user::{UserAddressRepository, UserRepository},
};
use crate::{dto::user::{AddressDto, UpsertAddressInput}, error::AppError};

pub struct UpsertAddress<UR, AR> {
    pub user_repo:    Arc<UR>,
    pub address_repo: Arc<AR>,
}

impl<UR, AR> UpsertAddress<UR, AR>
where
    UR: UserRepository,
    AR: UserAddressRepository,
{
    pub async fn add(
        &self,
        user_id: Uuid,
        input: UpsertAddressInput,
    ) -> Result<AddressDto, AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        // Verify user exists
        self.user_repo.find_by_id(user_id).await?;

        let is_default = input.is_default.unwrap_or(false);
        if is_default {
            // Will be set in set_default call below
        }

        let address = UserAddress {
            id:             Uuid::new_v4(),
            user_id,
            label:          input.label.unwrap_or_else(|| "Home".into()),
            recipient_name: input.recipient_name,
            phone:          input.phone,
            street:         input.street,
            ward:           input.ward,
            district:       input.district,
            city:           input.city,
            country:        "VN".into(),
            is_default:     false,
            created_at:     Utc::now(),
        };

        let address = self.address_repo.create(&address).await?;

        if is_default {
            self.address_repo.set_default(user_id, address.id).await?;
        }

        Ok(to_dto(address))
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        address_id: Uuid,
        input: UpsertAddressInput,
    ) -> Result<AddressDto, AppError> {
        input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        let mut address = self.address_repo.find_by_id(address_id).await?;
        if address.user_id != user_id {
            return Err(AppError::Unauthorized("Address does not belong to user".into()));
        }

        address.label          = input.label.unwrap_or(address.label);
        address.recipient_name = input.recipient_name;
        address.phone          = input.phone;
        address.street         = input.street;
        address.ward           = input.ward;
        address.district       = input.district;
        address.city           = input.city;

        let address = self.address_repo.update(&address).await?;

        if input.is_default.unwrap_or(false) {
            self.address_repo.set_default(user_id, address.id).await?;
        }

        Ok(to_dto(address))
    }
}

fn to_dto(a: UserAddress) -> AddressDto {
    AddressDto {
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
    }
}
