use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::{dto::{project_status_dto::{ReqCreateProjectStatusDto, ReqUpdateProjectStatusDto, ResEntryProjectStatusDto, ResListProjectStatusDto, ResUpdateProjectStatusDto}, std_201::ResCreateSuccess}, repository::require_implementation::trait_project_status::ProjectStatusRepoReqImpl};




pub struct ProjectStatusUseCase<T>
where
    T: ProjectStatusRepoReqImpl + Send + Sync,
{
    project_status_repo: Arc<T>,
}

#[derive(Debug, Error)]
pub enum ProjectStatusUseCaseError {
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



impl<T> ProjectStatusUseCase<T>
where
    T: ProjectStatusRepoReqImpl + Send + Sync,
{
    pub fn new(project_status_repo: Arc<T>) -> Self {
        ProjectStatusUseCase { project_status_repo }
    }

    pub async fn create_project_status(
        &self, 
        user_id: Uuid, 
        project_status_data: ReqCreateProjectStatusDto
    ) -> Result<ResCreateSuccess, ProjectStatusUseCaseError> {
        match self.project_status_repo.create_project_status(user_id, project_status_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_project_status(
        &self, user_id: Uuid, 
        project_status_id: Uuid
    ) -> Result<ResEntryProjectStatusDto, ProjectStatusUseCaseError> {
        match self.project_status_repo.get_project_status(user_id, project_status_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_project_statuses(
        &self, 
        user_id: Uuid
    ) -> Result<ResListProjectStatusDto, ProjectStatusUseCaseError> {
        match self.project_status_repo.get_project_statuses(user_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn update_project_status(
        &self, user_id: Uuid, 
        project_status_id: Uuid, 
        project_status_data: ReqUpdateProjectStatusDto
    ) -> Result<ResUpdateProjectStatusDto, ProjectStatusUseCaseError> {
        match self.project_status_repo.update_project_status(user_id, project_status_id, project_status_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn delete_project_status(
        &self, 
        user_id: Uuid, 
        project_status_id: Uuid
    ) -> Result<(), ProjectStatusUseCaseError> {
        match self.project_status_repo.delete_project_status(user_id, project_status_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }
}