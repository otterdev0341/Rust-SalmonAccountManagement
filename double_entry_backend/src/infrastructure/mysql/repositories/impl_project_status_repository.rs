use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use uuid::Uuid;

use crate::{application::usecase::project_status_usecase::ProjectStatusUseCaseError, domain::{dto::{project_status_dto::{ReqCreateProjectStatusDto, ReqUpdateProjectStatusDto, ResEntryProjectStatusDto, ResListProjectStatusDto, ResUpdateProjectStatusDto}, std_201::ResCreateSuccess}, entities::project_status, repository::require_implementation::trait_project_status::{ProjectStatusRepoReqImpl, ProjectStatusRepositoryError}}};



pub struct ImplProjectStatusRepository {
    pub db: Arc<DatabaseConnection>
}

impl ImplProjectStatusRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplProjectStatusRepository {
            db
        }
    }
}


impl From<ProjectStatusRepositoryError> for ProjectStatusUseCaseError {
    fn from(error: ProjectStatusRepositoryError) -> Self {
        match error {
            ProjectStatusRepositoryError::ProjectStatusNotFound => ProjectStatusUseCaseError::ProjectStatusNotFound,
            ProjectStatusRepositoryError::InternalServerError => ProjectStatusUseCaseError::InternalServerError,         
            ProjectStatusRepositoryError::DeleteFailed => ProjectStatusUseCaseError::DeleteFailed,
            ProjectStatusRepositoryError::CreateFailed => ProjectStatusUseCaseError::CreateFailed,
            ProjectStatusRepositoryError::UpdateFailed => ProjectStatusUseCaseError::UpdateFailed,
            ProjectStatusRepositoryError::ThisProjectNameAlreadyExist => ProjectStatusUseCaseError::ThisProjectNameAlreadyExist
        }
    }
}


#[async_trait::async_trait]
impl ProjectStatusRepoReqImpl for ImplProjectStatusRepository {
    async fn create_project_status(
        &self, 
        user_id: Uuid, 
        project_status_data: ReqCreateProjectStatusDto
    ) -> Result<ResCreateSuccess, ProjectStatusRepositoryError>
    {
        // extract and create new to insert
        let new_project = project_status::ActiveModel{
            name: Set(project_status_data.name),
            user_id: Set(user_id.as_bytes().to_vec()),
            description: Set(project_status_data.description),
            ..Default::default()
        };
        // try to insert
        let insert_result = project_status::Entity::insert(new_project).exec_with_returning(&*self.db).await;
        // check the error
        match insert_result {
            Ok(data) => {
                Ok(ResCreateSuccess{
                    id_created: Uuid::from_slice(&data.id).unwrap()
                })
            },
            Err(_) => {
                Err(ProjectStatusRepositoryError::CreateFailed)
            }
        }

        
    }

    async fn get_project_status(
        &self, user_id: Uuid, 
        project_status_id: Uuid
    ) -> Result<ResEntryProjectStatusDto, ProjectStatusRepositoryError>
    {
        // try to find it
        let find_record = project_status::Entity::find_by_id(project_status_id)
            .filter(project_status::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        // if error on find process
        if find_record.is_err() {
            return Err(ProjectStatusRepositoryError::InternalServerError);
        }
        // check is dontain record or not
        if let Some(data) = find_record.unwrap() {
            let res = ResEntryProjectStatusDto {
                id: Uuid::from_slice(&data.id).unwrap(),
                name: data.name.clone(),
                description: data.description.clone(),
                user_id: Uuid::from_slice(&data.user_id).unwrap(),
                created_at: data.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: data.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            };
            Ok(res)
        } else {
            Err(ProjectStatusRepositoryError::ProjectStatusNotFound)
        }

    }
    
    async fn get_project_statuses(
        &self, 
        user_id: Uuid
    ) -> Result<ResListProjectStatusDto, ProjectStatusRepositoryError>
    {
        let record_result = project_status::Entity::find()
            .filter(project_status::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .all(&*self.db)
            .await;
        if record_result.is_err() {
            return Err(ProjectStatusRepositoryError::InternalServerError);
        }
        let data = record_result.unwrap();
        let mut res_data = vec![];
        for item in data {
            res_data.push(ResEntryProjectStatusDto {
                id: Uuid::from_slice(&item.id).unwrap(),
                name: item.name.clone(),
                description: item.description.clone(),
                user_id: Uuid::from_slice(&item.user_id).unwrap(),
                created_at: item.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: item.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            });
        }
        Ok(ResListProjectStatusDto {
            total: res_data.len() as u32,
            statuses: res_data
        })
    }
    
    async fn update_project_status(
        &self, 
        user_id: Uuid, 
        project_status_id: Uuid, 
        project_status_data: ReqUpdateProjectStatusDto
    ) -> Result<ResUpdateProjectStatusDto, ProjectStatusRepositoryError>
    {
        let find_record = project_status::Entity::find_by_id(project_status_id)
            .filter(project_status::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        if find_record.is_err() {
            return Err(ProjectStatusRepositoryError::ProjectStatusNotFound);
        }
        
        if let Some(data) = find_record.unwrap() {
            let mut updated_record = data.into_active_model();
            if let Some(name) = project_status_data.name {
                updated_record.name = Set(name);
            }
            if let Some(description) = project_status_data.description {
                updated_record.description = Set(description);
            }
            let update_result = updated_record.update(&*self.db).await;
            match update_result {
                Ok(model) => {
                    let res = ResUpdateProjectStatusDto {
                        id: Uuid::from_slice(&model.id).unwrap(),
                        name: model.name.clone(),
                        description: model.description.clone(),
                        user_id: Uuid::from_slice(&model.user_id).unwrap(),
                        updated_at: model.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
                    };
                    return Ok(res);
                },
                Err(_) => {
                    return Err(ProjectStatusRepositoryError::UpdateFailed);
                }
            }
            

        } else {
            Err(ProjectStatusRepositoryError::ProjectStatusNotFound)
        }
    }
    
    async fn delete_project_status(
        &self, 
        user_id: Uuid, 
        project_status_id: Uuid
    ) -> Result<(), ProjectStatusRepositoryError>
    {
        // try to find it
        let find_target = project_status::Entity::find_by_id(project_status_id)
            .filter(project_status::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        if find_target.is_err() {
            return Err(ProjectStatusRepositoryError::InternalServerError);
        }
        if let Some(data) = find_target.unwrap() {
            let delete_result = data.delete(&*self.db).await;
            if delete_result.is_err() {
                return Err(ProjectStatusRepositoryError::DeleteFailed);
            } else {
                return Ok(());
            }
        } else {
            return Err(ProjectStatusRepositoryError::ProjectStatusNotFound);
        }
    }
}