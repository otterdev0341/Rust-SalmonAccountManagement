use std::sync::Arc;

use crate::{domain::{dto::auth_dto::ReqCreateUserDto, repository::require_implementation::trait_auth::AuthRepoReqImpl}, infrastructure::handler::operation_status::auth_error::CreateUserError};





pub struct AuthUseCase<T>
where
    T: AuthRepoReqImpl + Send + Sync,
{
    auth_repo: Arc<T>,
}


impl<T> AuthUseCase<T>
where
    T: AuthRepoReqImpl + Send + Sync,
{
    pub fn new(auth_repo: Arc<T>) -> Self {
        AuthUseCase { auth_repo }
    }

    pub async fn create_user(&self, user_data: ReqCreateUserDto) -> Result<(), CreateUserError> {
        match self.auth_repo.create_user(user_data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}