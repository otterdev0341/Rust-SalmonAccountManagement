use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::{dto::{contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto, ResUpdateContactDto}, std_201::ResCreateSuccess}, repository::require_implementation::trait_contact::ContactRepoReqImpl};






pub struct ContactUseCase<T>
where
    T: ContactRepoReqImpl + Send + Sync,
{
    pub contact_repo: Arc<T>,
}

#[derive(Debug, Error)]
pub enum ContactUseCaseError{
    #[error("Contact not found")]
    ContactNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Not allow to peform operation")]
    UnAuthorized,
}

impl<T> ContactUseCase<T>
where
    T: ContactRepoReqImpl + Send + Sync,
{
    pub fn new(contact_repo: Arc<T>) -> Self {
        ContactUseCase { contact_repo }
    }

    pub async fn create_contact(&self, user_id: Uuid, contact_data: ReqCreateContactDto) -> Result<ResCreateSuccess, ContactUseCaseError> {
        match self.contact_repo.create_contact(user_id, contact_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_contact(&self, user_id: Uuid, contact_id: Uuid) -> Result<ResEntryContactDto, ContactUseCaseError> {
        match self.contact_repo.get_contact(user_id, contact_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_contacts(&self, user_id: Uuid) -> Result<ResListContactDto, ContactUseCaseError> {
        match self.contact_repo.get_contacts(user_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn update_contact(&self, user_id: Uuid, contact_id: Uuid, contact_data: ReqUpdateContactDto) -> Result<ResUpdateContactDto, ContactUseCaseError> {
        match self.contact_repo.update_contact(user_id, contact_id, contact_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn delete_contact(&self, user_id: Uuid, contact_id: Uuid) -> Result<(), ContactUseCaseError> {
        match self.contact_repo.delete_contact(user_id, contact_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }
    
}