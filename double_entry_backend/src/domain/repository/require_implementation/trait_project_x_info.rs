use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::dto::project_x_info_dto::ResAddInfoToProjectDto;


#[derive(Debug, Error)]
pub enum ProjectRepositoryError {
    #[error("This operation is can not be performed")]
    UnAuthorized,
    #[error("Internal server error")]
    InternalServerError,
}

#[async_trait::async_trait]
pub trait ProjectXInfoRepoReqImpl {
     fn add_info_to_project(&self, user_id: Uuid, project_id: Uuid, info_id: Uuid) -> Result<ResAddInfoToProjectDto, ProjectRepositoryError>;
     fn remove_info_from_project(&self, user_id: Uuid, project_id: Uuid, info_id: Uuid) -> Result<(), ProjectRepositoryError>;
}