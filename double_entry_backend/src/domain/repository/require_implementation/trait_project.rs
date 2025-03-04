use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::dto::{project_dto::{ReqCreateProjectDto, ReqUpdateProjectDto, ResEntryProjectDto, ResListProjectDto, ResUpdateProjectDto}, std_201::ResCreateSuccess};


#[derive(Debug, Error)]
pub enum ProjectRepositoryError {
    #[error("Project not found")]
    ProjectNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Duplicate project")]
    ConflictingProject,
    #[error("Delete failed")]
    DeleteFailed,
    #[error("Update failed")]
    UpdateFailed
}


#[async_trait::async_trait]
pub trait ProjectRepoReqImpl {
    async fn create_project(
        &self, 
        user_id: Uuid, 
        company_id: Uuid,
        project_data: ReqCreateProjectDto,
    ) -> Result<ResCreateSuccess, ProjectRepositoryError>;

    async fn get_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid
    ) -> Result<ResEntryProjectDto, ProjectRepositoryError>;
    
    async fn get_projects(
        &self, 
        user_id: Uuid
    ) -> Result<ResListProjectDto, ProjectRepositoryError>;
    
    async fn update_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid, 
        project_data: ReqUpdateProjectDto
    ) -> Result<ResUpdateProjectDto, ProjectRepositoryError>;
    
    async fn delete_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid
    ) -> Result<(), ProjectRepositoryError>;

}