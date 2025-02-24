use std::{sync::Arc, time::SystemTime};

use crate::{domain::{dto::auth_dto::{ClaimsDto, ReqCreateUserDto, ReqSignInDto, ResSignInDto}, repository::require_implementation::trait_auth::AuthRepoReqImpl}, infrastructure::{handler::operation_status::auth_error::{CreateUserError, SignInError}, jwt_service::jwt::generate_jwt}};





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







    pub async fn sign_in(&self, user_data: ReqSignInDto) -> Result<ResSignInDto, SignInError>{

        let user = self.auth_repo.sign_in(user_data).await;

        match user {
            Ok(user) => {
                let jwt = generate_jwt(user.id, &user.username);
                match jwt {
                    Ok(token) => Ok(token),
                    Err(_) => Err(SignInError::InternalServerError)
                }
            },
            Err(e) => Err(SignInError::InternalServerError)
        }


    }
}