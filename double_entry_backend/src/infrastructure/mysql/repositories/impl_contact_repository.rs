use std::sync::Arc;

use sea_orm::DatabaseConnection;
use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::{application::usecase::contact_usecase::ContactUseCaseError, domain::{dto::{contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto, ResUpdateContactDto}, std_201::ResCreateSuccess}, repository::require_implementation::trait_contact::ContactRepoReqImpl}};






pub struct ImplContactRepository {
    pub db: Arc<DatabaseConnection>
}



impl ImplContactRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplContactRepository {
            db
        }
    }
}

#[derive(Debug, Error)]
pub enum ContactRepositoryError {
    #[error("Contact not found")]
    ContactNotFound,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Duplicate contact")]
    UnAuthorized,
}

impl From<ContactRepositoryError> for ContactUseCaseError {
    fn from(error: ContactRepositoryError) -> Self {
        match error {
            ContactRepositoryError::ContactNotFound => ContactUseCaseError::ContactNotFound,
            ContactRepositoryError::InternalServerError => ContactUseCaseError::InternalServerError,
            ContactRepositoryError::UnAuthorized => ContactUseCaseError::UnAuthorized,
        }
    }
}

#[async_trait::async_trait]
impl ContactRepoReqImpl for ImplContactRepository{
    async fn create_contact(
        &self, 
        user_id: Uuid, 
        contact_data: ReqCreateContactDto
    ) 
    -> Result<ResCreateSuccess, ContactRepositoryError> {
        unimplemented!()
    }

    async fn get_contact(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) -> Result<ResEntryContactDto, ContactRepositoryError> {
        unimplemented!()
    }

    async fn get_contacts(
        &self, 
        user_id: Uuid
    ) -> Result<ResListContactDto, ContactRepositoryError> {
        unimplemented!()
    }

    async fn update_contact(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid, 
        contact_data: ReqUpdateContactDto
    ) -> Result<ResUpdateContactDto, ContactRepositoryError> {
        unimplemented!()
    }

    async fn delete_contact(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) -> Result<(), ContactRepositoryError> {
        unimplemented!()
    }
}