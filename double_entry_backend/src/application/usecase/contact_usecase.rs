use std::sync::Arc;

use thiserror::Error;

use crate::domain::repository::require_implementation::trait_contact::ContactRepoReqImpl;






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

    
}