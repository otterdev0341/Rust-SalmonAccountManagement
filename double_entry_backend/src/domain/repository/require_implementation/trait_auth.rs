
use sea_orm_migration::async_trait;

use crate::{domain::dto::{auth_dto::{ReqCreateUserDto, ReqSignInDto, ResEntryUserDto, ResSignInDto}, std_201::ResCreateSuccess}, infrastructure::mysql::repositories::impl_auth_repository::AuthRepositoryError};




#[async_trait::async_trait]
pub trait AuthRepoReqImpl {
    async fn create_user(&self, user_data: ReqCreateUserDto) -> Result<ResCreateSuccess, AuthRepositoryError>;
    async fn sign_in(&self, sign_data: ReqSignInDto) -> Result<ResEntryUserDto, AuthRepositoryError>;
}