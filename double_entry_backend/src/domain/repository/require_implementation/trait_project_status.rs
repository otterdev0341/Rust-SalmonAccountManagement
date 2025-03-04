use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::dto::{project_status_dto::{ReqCreateProjectStatusDto, ReqUpdateProjectStatusDto, ResEntryProjectStatusDto, ResListProjectStatusDto, ResUpdateProjectStatusDto}, std_201::ResCreateSuccess};


#[derive(Debug, Error)]
pub enum ProjectStatusRepositoryError {
    #[error("Project Status not found")]
    ProjectStatusNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("UnAuthorized")]
    DeleteFailed,
    #[error("CreateFaild failed")]
    CreateFailed,
    #[error("Update failed")]
    UpdateFailed,
    #[error("This project status name already exist")]
    ThisProjectNameAlreadyExist
}


#[async_trait::async_trait]
pub trait ProjectStatusRepoReqImpl {
    async fn create_project_status(
        &self, 
        user_id: Uuid, 
        project_status_data: ReqCreateProjectStatusDto
    ) -> Result<ResCreateSuccess, ProjectStatusRepositoryError>;

    async fn get_project_status(
        &self, user_id: Uuid, 
        project_status_id: Uuid
    ) -> Result<ResEntryProjectStatusDto, ProjectStatusRepositoryError>;
    
    async fn get_project_statuses(
        &self, 
        user_id: Uuid
    ) -> Result<ResListProjectStatusDto, ProjectStatusRepositoryError>;
    
    async fn update_project_status(
        &self, user_id: Uuid, 
        project_status_id: Uuid, 
        project_status_data: ReqUpdateProjectStatusDto
    ) -> Result<ResUpdateProjectStatusDto, ProjectStatusRepositoryError>;
    
    async fn delete_project_status(
        &self, 
        user_id: Uuid, 
        project_status_id: Uuid
    ) -> Result<(), ProjectStatusRepositoryError>;
}