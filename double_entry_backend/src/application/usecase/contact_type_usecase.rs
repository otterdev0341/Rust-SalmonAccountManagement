use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::{dto::{contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto, ResEntryContactTypeDto, ResListContactTypeDto, ResUpdateContactTypeDto}, std_201::ResCreateSuccess}, repository::require_implementation::trait_contact_type::ContactTypeRepoReqimpl};





pub struct ContactTypeUseCase<T>{
    pub contact_type_repo: Arc<T>,
}

#[derive(Debug, Error)]
pub enum ContactTypeUseCaseError{
    #[error("Contact type not found")]
    ContactTypeNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Not allow to peform operation")]
    UnAuthorized,
    #[error("Fail to delete contact type")]
    DeleteFailed,
    #[error("Fail to update contact type")]
    UpdateFailed,   
}

impl<T> ContactTypeUseCase<T>
where
    T: ContactTypeRepoReqimpl + Send + Sync,
{
    pub fn new(contact_type_repo: Arc<T>) -> Self {
        ContactTypeUseCase { contact_type_repo }
    }

    pub async fn create_contact_type(&self, user_id: Uuid, contact_type_data: ReqCreateContactTypeDto) -> Result<ResCreateSuccess, ContactTypeUseCaseError> {
        match self.contact_type_repo.create_contact_type(user_id, contact_type_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_contact_type(&self, user_id: Uuid, contact_type_id: Uuid) -> Result<ResEntryContactTypeDto, ContactTypeUseCaseError> {
        match self.contact_type_repo.get_contact_type(user_id, contact_type_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_contact_types(&self, user_id: Uuid) -> Result<ResListContactTypeDto, ContactTypeUseCaseError> {
        match self.contact_type_repo.get_contact_types(user_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn update_contact_type(&self, user_id: Uuid, contact_type_id: Uuid, contact_type_data: ReqUpdateContactTypeDto) -> Result<ResUpdateContactTypeDto, ContactTypeUseCaseError> {
        match self.contact_type_repo.update_contact_type(user_id, contact_type_id, contact_type_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn delete_contact_type(&self, user_id: Uuid, contact_type_id: Uuid) -> Result<(), ContactTypeUseCaseError> {
        match self.contact_type_repo.delete_contact_type(user_id, contact_type_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }
}