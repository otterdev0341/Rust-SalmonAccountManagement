
use sea_orm_migration::async_trait;

use crate::{domain::dto::auth_dto::{ReqCreateUserDto, ReqSignInDto, ResEntryUserDto, ResSignInDto}, infrastructure::{handler::operation_status::auth_error::{CreateUserError, SignInError}, mysql::repositories::impl_auth_repository::AuthRepositoryError}};



#[async_trait::async_trait]
pub trait AuthRepoReqImpl {
    async fn create_user(&self, user_data: ReqCreateUserDto) -> Result<(), AuthRepositoryError>;
    async fn sign_in(&self, sign_data: ReqSignInDto) -> Result<ResEntryUserDto, AuthRepositoryError>;
}