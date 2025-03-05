use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::dto::project_x_info_dto::{ResInfoProjectDto, ResInfosOfProjectDto, ResProjectsOfInfoDto};




#[derive(Debug, Error)]
pub enum ProjectXInfoRepositoryError {
    #[error("This operation is can not be performed")]
    UnAuthorized,
    #[error("Internal server error")]
    InternalServerError,
}


#[async_trait::async_trait]
pub trait ProjectXInfoRepoReqImpl {
     async fn add_info_to_project(&self, user_id: Uuid, project_id: Uuid, info_id: Uuid) -> Result<ResInfoProjectDto, ProjectXInfoRepositoryError>;
     async fn remove_info_from_project(&self, user_id: Uuid, project_id: Uuid, info_id: Uuid) -> Result<(), ProjectXInfoRepositoryError>;
     async fn get_all_infos_of_project(&self, user_id: Uuid, project_id: Uuid) -> Result<ResInfosOfProjectDto, ProjectXInfoRepositoryError>;
     async fn get_all_projects_of_info(&self, user_id: Uuid, info_id: Uuid) -> Result<ResProjectsOfInfoDto, ProjectXInfoRepositoryError>;
}