use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::dto::{info_dto::{ReqCreateInfoDto, ReqUpdateInfoDto, ResEntryInfoDto, ResListInfoDto, ResUpdateInfoDto}, std_201::ResCreateSuccess};

#[derive(Debug, Error)]
pub enum InfoRepositoryError {
    #[error("Info not found")]
    InfoNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Failed to delete info")]
    DeleteFailed,
    #[error("Failed to update info")]
    UpdateFailed,
    #[error("Failed to create info")]
    CreateFailed
}

#[async_trait::async_trait]
pub trait InfoRepoReqImpl {
    async fn create_info(
        &self, 
        user_id: Uuid, 
        info_data: ReqCreateInfoDto
    ) -> Result<ResCreateSuccess, InfoRepositoryError>;

    async fn get_info(
        &self, 
        user_id: Uuid
    ) -> Result<ResEntryInfoDto, InfoRepositoryError>;

    async fn get_infos(
        &self, 
        user_id: Uuid
    ) -> Result<ResListInfoDto, InfoRepositoryError>;
    
    async fn update_info(
        &self, 
        user_id: Uuid, 
        info_data: ReqUpdateInfoDto
    ) -> Result<ResUpdateInfoDto, InfoRepositoryError>;

    async fn delete_info(
        &self, 
        user_id: Uuid
    ) -> Result<(), InfoRepositoryError>;
}