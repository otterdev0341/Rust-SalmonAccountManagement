use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::{dto::{project_dto::{ReqCreateProjectDto, ReqUpdateProjectDto, ResEntryProjectDto, ResListProjectDto, ResUpdateProjectDto}, std_201::ResCreateSuccess}, repository::require_implementation::trait_project::ProjectRepoReqImpl};



pub struct ProjectUseCase<T> 
where 
    T: ProjectRepoReqImpl + Send + Sync,
{
    project_repo: Arc<T>,
}

#[derive(Debug, Error)]
pub enum ProjectUseCaseError{
    #[error("Company not found")]
    ProjectNotFound,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Duplicate company")]
    ConflictingCompany,

    #[error("Uuid convert error")]
    ConvertUuidError,

    #[error("Delete failed")]
    DeleteFailed,

    #[error("Update failed")]
    UpdateFailed,
}

impl<T> ProjectUseCase<T> 
where 
    T: ProjectRepoReqImpl + Send + Sync,
{
    pub fn new(project_repo: Arc<T>) -> Self {
        ProjectUseCase { project_repo }
    }

    pub async fn create_project(
        &self, 
        user_id: Uuid, 
        company_id: Uuid, 
        project_data: ReqCreateProjectDto
    ) 
    -> Result<ResCreateSuccess, ProjectUseCaseError> 
    {
        match self.project_repo.create_project(user_id, company_id, project_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid
    ) 
    -> Result<ResEntryProjectDto, ProjectUseCaseError> 
    {
        match self.project_repo.get_project(user_id, project_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_projects(
        &self, 
        user_id: Uuid
    ) 
    -> Result<ResListProjectDto, ProjectUseCaseError> 
    {
        match self.project_repo.get_projects(user_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn update_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid, 
        project_data: ReqUpdateProjectDto
    ) 
    -> Result<ResUpdateProjectDto, ProjectUseCaseError> 
    {
        match self.project_repo.update_project(user_id, project_id, project_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn delete_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid
    ) 
    -> Result<(), ProjectUseCaseError> 
    {
        match self.project_repo.delete_project(user_id, project_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }
}