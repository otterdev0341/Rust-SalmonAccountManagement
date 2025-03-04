use std::sync::Arc;

use rocket::data::N;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use uuid::Uuid;

use crate::{application::usecase::project_usecase::ProjectUseCaseError, domain::{dto::{project_dto::{ReqCreateProjectDto, ReqUpdateProjectDto, ResEntryProjectDto, ResListProjectDto, ResUpdateProjectDto}, std_201::ResCreateSuccess}, entities::project, repository::require_implementation::trait_project::{ProjectRepoReqImpl, ProjectRepositoryError}}};



pub struct ImplProjectRepository {
    pub db: Arc<DatabaseConnection>
}

impl ImplProjectRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplProjectRepository {
            db
        }
    }
}

impl From<ProjectRepositoryError> for ProjectUseCaseError{
    fn from(error: ProjectRepositoryError) -> Self {
        match error {
            ProjectRepositoryError::ProjectNotFound => ProjectUseCaseError::ProjectNotFound,
            ProjectRepositoryError::InternalServerError => ProjectUseCaseError::InternalServerError,
            ProjectRepositoryError::ConflictingProject => ProjectUseCaseError::ConflictingCompany,
            ProjectRepositoryError::DeleteFailed => ProjectUseCaseError::DeleteFailed,
            ProjectRepositoryError::UpdateFailed => ProjectUseCaseError::UpdateFailed
        }
    }
}



#[async_trait::async_trait]
impl ProjectRepoReqImpl for ImplProjectRepository {
    async fn create_project(
        &self, 
        user_id: Uuid, 
        company_id: Uuid,
        project_data: ReqCreateProjectDto
    ) -> Result<ResCreateSuccess, ProjectRepositoryError>
    {
        let mut new_project = project::ActiveModel {
            company_id: Set(company_id.as_bytes().to_vec()),
            name: Set(project_data.name),
            description: Set(project_data.description),
            ..Default::default()
        };
        
        if let Some(project_id) = project_data.project_status_id {
            new_project.project_status_id = Set(project_id.as_bytes().to_vec());
        }

        let insert_result = project::Entity::insert(new_project).exec_with_returning(&*self.db).await;
        match insert_result {
            Ok(data) => {
                Ok(ResCreateSuccess {
                    id_created: Uuid::from_slice(&data.id).unwrap()
                })
            },
            Err(_) => Err(ProjectRepositoryError::InternalServerError)
        }

    }

    async fn get_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid
    ) -> Result<ResEntryProjectDto, ProjectRepositoryError>
    {
            // find with user and project_id
            let find_result = project::Entity::find_by_id(project_id)
                .filter(project::Column::CompanyId.eq(user_id.as_bytes().to_vec()))
                .one(&*self.db)
                .await;
            if find_result.is_err() {
                return Err(ProjectRepositoryError::InternalServerError)
            }
            if let Some(project) = find_result.unwrap() {
                Ok(ResEntryProjectDto {
                    id: Uuid::from_slice(&project.id).unwrap(),
                    company_id: Uuid::from_slice(&project.company_id).unwrap(),
                    user_id: Uuid::from_slice(&project.user_id).unwrap(),
                    name: project.name.clone(),
                    description: project.description.clone(),
                    project_status_id: Some(Uuid::from_slice(&project.project_status_id).unwrap()),
                    created_at: project.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                    updated_at: project.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
                })
            } else {
                Err(ProjectRepositoryError::ProjectNotFound)
            }
            
    }
    
    async fn get_projects(
        &self, 
        user_id: Uuid
    ) -> Result<ResListProjectDto, ProjectRepositoryError>
    {
            let list_result = project::Entity::find()
                .filter(project::Column::CompanyId.eq(user_id.as_bytes().to_vec()))
                .all(&*self.db)
                .await;
            if list_result.is_err() {
                return Err(ProjectRepositoryError::InternalServerError)
            }
            let projects = list_result.unwrap();
            let mut res_projects = vec![];
            for project in projects {
                res_projects.push(ResEntryProjectDto {
                    id: Uuid::from_slice(&project.id).unwrap(),
                    company_id: Uuid::from_slice(&project.company_id).unwrap(),
                    user_id: Uuid::from_slice(&project.user_id).unwrap(),
                    name: project.name.clone(),
                    description: project.description.clone(),
                    project_status_id: Some(Uuid::from_slice(&project.project_status_id).unwrap()),
                    created_at: project.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                    updated_at: project.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
                });
            }
            Ok(ResListProjectDto {
                total: res_projects.len() as u32,
                projects: res_projects
            })
    }
    
    async fn update_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid, 
        project_data: ReqUpdateProjectDto
    ) -> Result<ResUpdateProjectDto, ProjectRepositoryError>
    {
            // find target
            let target = project::Entity::find_by_id(project_id)
                .filter(project::Column::CompanyId.eq(user_id.as_bytes().to_vec()))
                .one(&*self.db)
                .await;
            // handle error
            if target.is_err() {
                return Err(ProjectRepositoryError::InternalServerError)
            }
            // convert to mut active model
            match target.unwrap() {
                Some(target) => {
                    let mut prepare_update = target.into_active_model();
                    if let Some(name) = project_data.name {
                        prepare_update.name = Set(name);
                    }
                    if let Some(description) = project_data.description {
                        prepare_update.description = Set(description);
                    }
                    if let Some(project_status_id) = project_data.project_status_id {
                        prepare_update.project_status_id = Set(project_status_id.as_bytes().to_vec());
                    }
                    // update
                    let update_result = prepare_update.update(&*self.db).await;
                    if update_result.is_err() {
                        return Err(ProjectRepositoryError::UpdateFailed)
                    }
                    let extract_inner = update_result.unwrap();
                    Ok(ResUpdateProjectDto {
                        id: Uuid::from_slice(&extract_inner.id).unwrap(),
                        company_id: Uuid::from_slice(&extract_inner.company_id).unwrap(),
                        user_id: Uuid::from_slice(&extract_inner.user_id).unwrap(),
                        name: extract_inner.name.clone(),
                        description: extract_inner.description.clone(),
                        project_status_id: Some(Uuid::from_slice(&extract_inner.project_status_id).unwrap()),
                        
                        updated_at: extract_inner.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
                    })
                },
                None => Err(ProjectRepositoryError::ProjectNotFound)
            }       
    }
    
    async fn delete_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid
    ) -> Result<(), ProjectRepositoryError>
    {
            let target = project::Entity::find_by_id(project_id)
                .filter(project::Column::CompanyId.eq(user_id.as_bytes().to_vec()))
                .one(&*self.db)
                .await;
            if target.is_err() {
                return Err(ProjectRepositoryError::InternalServerError)
            }
            if let Some(target) = target.unwrap() {
                let delete_result = target.delete(&*self.db).await;
                if delete_result.is_err() {
                    return Err(ProjectRepositoryError::DeleteFailed)
                } else {
                    Ok(())
                }
            } else {
                Err(ProjectRepositoryError::ProjectNotFound)
            }
    }
    


}