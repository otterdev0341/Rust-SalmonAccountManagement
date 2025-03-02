use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::{dto::{contact_detail_dto::{ReqCreateContactDetailDto, ReqUpdateContactDetailDto, ResEntryContactDetailDto, ResListContactDetailDto, ResUpdateContactDetailDto}, std_201::ResCreateSuccess}, repository::require_implementation::trait_contact_detail::ContactDetailRepoReqImpl};

pub struct ContactDetailUseCase<T>
where 
    T: ContactDetailRepoReqImpl + Send + Sync,
{
    pub contact_detail_repo: Arc<T>,
}

#[derive(Debug, Error)]
pub enum ContactDetailUseCaseError {
    #[error("Contact detail not found")]
    ContactDetailNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Fail to delete contact detail")]
    DeleteFailed,
    #[error("Fail to update contact detail")]
    UpdateFailed,
}

impl<T> ContactDetailUseCase<T>
where
    T: ContactDetailRepoReqImpl + Send + Sync,
{
    pub fn new(contact_detail_repo: Arc<T>) -> Self {
        ContactDetailUseCase {
            contact_detail_repo
        }
    }

    pub async fn create_contact_detail(
        &self,
        user_id: Uuid,
        contact_id: Uuid,
        contact_detail_data: ReqCreateContactDetailDto
    ) -> Result<ResCreateSuccess, ContactDetailUseCaseError> {
        match self.contact_detail_repo.create_contact_detail(user_id, contact_id, contact_detail_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_contact_detail(
        &self,
        user_id: Uuid,
        contact_id: Uuid
    ) -> Result<ResEntryContactDetailDto, ContactDetailUseCaseError> {
        match self.contact_detail_repo.get_contact_detail(user_id, contact_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_contact_details(
        &self,
        user_id: Uuid
    ) -> Result<ResListContactDetailDto, ContactDetailUseCaseError> {
        match self.contact_detail_repo.get_contact_details(user_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn update_contact_detail(
        &self,
        user_id: Uuid,
        contact_id: Uuid,
        contact_detail_data: ReqUpdateContactDetailDto
    ) -> Result<ResUpdateContactDetailDto, ContactDetailUseCaseError> {
        match self.contact_detail_repo.update_contact_detail(user_id, contact_id, contact_detail_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn delete_contact_detail(
        &self,
        user_id: Uuid,
        contact_id: Uuid
    ) -> Result<(), ContactDetailUseCaseError> {
        match self.contact_detail_repo.delete_contact_detail(user_id, contact_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

}