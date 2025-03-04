use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::{dto::{info_dto::{ReqCreateInfoDto, ReqUpdateInfoDto, ResEntryInfoDto, ResListInfoDto, ResUpdateInfoDto}, std_201::ResCreateSuccess}, repository::require_implementation::trait_info::InfoRepoReqImpl};


pub struct InfoUseCase<T>
where
    T: InfoRepoReqImpl + Send + Sync,
{
    info_repo: Arc<T>,
}



#[derive(Debug, Error)]
pub enum InfoUseCaseError {
    #[error("Info not found")]
    InfoNotFound,
    
    #[error("Internal server error")]
    InternalServerError,
    
    #[error("Duplicate info")]
    ConflictingInfo,

    
    #[error("Delete failed")]
    DeleteFailed,
    
    #[error("Update failed")]
    UpdateFailed,
}


impl<T> InfoUseCase<T>
where
    T: InfoRepoReqImpl + Send + Sync,
{
    pub fn new(info_repo: Arc<T>) -> Self {
        InfoUseCase { info_repo }
    }

    pub async fn create_info(&self, user_id: Uuid,info_data: ReqCreateInfoDto) -> Result<ResCreateSuccess, InfoUseCaseError> {
        match self.info_repo.create_info(user_id, info_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_info(&self,user_id: Uuid, info_id: Uuid) -> Result<ResEntryInfoDto, InfoUseCaseError> {
        match self.info_repo.get_info(user_id, info_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_infos(&self, user_id: Uuid) -> Result<ResListInfoDto, InfoUseCaseError> {
        match self.info_repo.get_infos(user_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn update_info(&self, user_id: Uuid,info_id: Uuid, info_data: ReqUpdateInfoDto) -> Result<ResUpdateInfoDto, InfoUseCaseError> {
        match self.info_repo.update_info(user_id, info_id, info_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn delete_info(&self, user_id: Uuid,info_id: Uuid) -> Result<(), InfoUseCaseError> {
        match self.info_repo.delete_info(user_id, info_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }
}