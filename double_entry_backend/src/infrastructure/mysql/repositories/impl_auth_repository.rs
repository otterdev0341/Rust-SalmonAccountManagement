use std::sync::Arc;

use sea_orm::DatabaseConnection;
use sea_orm_migration::async_trait;

use crate::{domain::{dto::auth_dto::{ReqCreateUserDto, ReqSignInDto, ResEntryUserDto}, repository::require_implementation::trait_auth::AuthRepoReqImpl}, infrastructure::handler::operation_status::auth_error::{CreateUserError, SignInError}};



pub struct ImplAuthRepository {
    pub db: Arc<DatabaseConnection>
}

impl ImplAuthRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplAuthRepository {
            db
        }
    }
}


#[async_trait::async_trait]
impl AuthRepoReqImpl for ImplAuthRepository {
    async fn create_user(&self, user_data: ReqCreateUserDto) -> Result<(), CreateUserError> {
        unimplemented!()
    }

    async fn sign_in(&self, sign_data: ReqSignInDto) -> Result<ResEntryUserDto, SignInError> {
        unimplemented!()
    }
}