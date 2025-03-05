use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, QuerySelect, Set};
use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::{dto::{info_dto::ResEntryInfoDto, project_dto::ResEntryProjectDto, project_x_info_dto::{ResInfoProjectDto, ResInfosOfProjectDto, ResProjectsOfInfoDto}}, entities::{info, project, project_x_info}, repository::require_implementation::trait_project_x_info::{ProjectXInfoRepoReqImpl, ProjectXInfoRepositoryError}};


pub struct ImplProjectXInfoRepository {
    pub db: Arc<DatabaseConnection>
}

impl ImplProjectXInfoRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplProjectXInfoRepository {
            db
        }
    }
}



#[async_trait::async_trait]
impl ProjectXInfoRepoReqImpl for ImplProjectXInfoRepository{
    async fn add_info_to_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid, 
        info_id: Uuid
    ) 
    -> Result<ResInfoProjectDto, ProjectXInfoRepositoryError>{
        // find is relation exist
        let find_result = project_x_info::Entity::find()
            .filter(project_x_info::Column::ProjectId.eq(project_id.as_bytes().to_vec()))
            .filter(project_x_info::Column::InfoId.eq(info_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        
        // if exist return conflict
        if find_result.is_err() {
            return Err(ProjectXInfoRepositoryError::InternalServerError)
        }
        // if not exist create relation
        if let Some(_relation) = find_result.unwrap() {
            return Err(ProjectXInfoRepositoryError::InternalServerError)
        }
        let new_relation = project_x_info::ActiveModel {
            project_id: Set(project_id.as_bytes().to_vec()),
            info_id: Set(info_id.as_bytes().to_vec())
        };
        let insert_result = project_x_info::Entity::insert(new_relation)
            .exec_with_returning(&*self.db)
            .await;
        if insert_result.is_err() {
            return Err(ProjectXInfoRepositoryError::InternalServerError)
        }
        // if error return internal server error
        // if success return ResInfoProjectDto
        Ok(ResInfoProjectDto {
            message: "info added to project".to_string(),
            info_id,
            project_id
        })
        
    }



    async fn remove_info_from_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid, 
        info_id: Uuid
    ) 
    -> Result<(), ProjectXInfoRepositoryError>{
        // check is project id belong to this user
        let is_project_exist = project::Entity::find()
            .filter(project::Column::Id.eq(project_id.as_bytes().to_vec()))
            .filter(project::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        // check is info id belong to this user
        let is_info_exist = info::Entity::find()
            .filter(info::Column::Id.eq(info_id.as_bytes().to_vec()))
            .filter(info::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        // handle if one of them got error on check
        if is_project_exist.is_err() || is_info_exist.is_err() {
            return Err(ProjectXInfoRepositoryError::InternalServerError)
        }
        // if one of them not exist return not found
        if is_project_exist.unwrap().is_none() || is_info_exist.unwrap().is_none() {
            return Err(ProjectXInfoRepositoryError::UnAuthorized)
        }
        // check is it exist in relation
        let find_result = project_x_info::Entity::find()
            .filter(project_x_info::Column::ProjectId.eq(project_id.as_bytes().to_vec()))
            .filter(project_x_info::Column::InfoId.eq(info_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        // handle find error case
        if find_result.is_err() {
            return Err(ProjectXInfoRepositoryError::InternalServerError)
        }
        if let Some(inner) = find_result.unwrap() {
            // if exist delete it
            let delete_result = inner.delete(&*self.db).await;
            if delete_result.is_err() {
                return Err(ProjectXInfoRepositoryError::InternalServerError)
            }
            Ok(())
        } else {
            // if not exist return not found
            Err(ProjectXInfoRepositoryError::UnAuthorized)
        }

    }
        

    async fn get_all_infos_of_project(
        &self, 
        user_id: Uuid, 
        project_id: Uuid
    ) 
    -> Result<ResInfosOfProjectDto, ProjectXInfoRepositoryError>{
        //check is project id belong to this user
        let is_project_belong_to_user = project::Entity::find()
            .filter(project::Column::Id.eq(project_id.as_bytes().to_vec()))
            .filter(project::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        if is_project_belong_to_user.is_err() {
            return Err(ProjectXInfoRepositoryError::InternalServerError)
        }
        // if not return unauthorized
        if is_project_belong_to_user.unwrap().is_none() {
            return Err(ProjectXInfoRepositoryError::UnAuthorized)
        }
        
        // if belong to user return all info of project
        let infos = info::Entity::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                project_x_info::Entity::belongs_to(info::Entity)
                    .from(project_x_info::Column::InfoId)
                    .to(info::Column::Id)
                    .into(),
            )
            .filter(project_x_info::Column::ProjectId.eq(project_id.as_bytes().to_vec()))
            .all(&*self.db)
            .await;
        if infos.is_err() {
            return Err(ProjectXInfoRepositoryError::InternalServerError)
        }
        let mut res_infos = vec![];
        for info in infos.unwrap() {
            res_infos.push(ResEntryInfoDto {
                id: Uuid::from_slice(&info.id).unwrap(),
                user_id: Uuid::from_slice(&info.user_id).unwrap(),
                title: info.title.clone(),
                content: info.content.clone(),
                created_at: info.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: info.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            });
        }
        Ok(ResInfosOfProjectDto {
            total: res_infos.len() as u32,
            infos: res_infos
        })
        
    }


    
    async fn get_all_projects_of_info(
        &self, 
        user_id: Uuid, 
        info_id: Uuid
    ) 
    -> Result<ResProjectsOfInfoDto, ProjectXInfoRepositoryError>{
        //check is in id belong to this user
        let is_info_belong_to_user = info::Entity::find()
            .filter(info::Column::Id.eq(info_id.as_bytes().to_vec()))
            .filter(info::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        // if not return unauthorized
        if is_info_belong_to_user.is_err() {
            return Err(ProjectXInfoRepositoryError::InternalServerError)
        }
        let projects = project::Entity::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                project_x_info::Entity::belongs_to(project::Entity)
                    .from(project_x_info::Column::ProjectId)
                    .to(project::Column::Id)
                    .into(),
            )
            .filter(project_x_info::Column::InfoId.eq(info_id.as_bytes().to_vec()))
            .all(&*self.db)
            .await;
        if projects.is_err() {
            return Err(ProjectXInfoRepositoryError::InternalServerError)
        }
        let mut res_projects = vec![];
        for project in projects.unwrap() {
            res_projects.push(ResEntryProjectDto {
                id: Uuid::from_slice(&project.id).unwrap(),
                user_id: Uuid::from_slice(&project.user_id).unwrap(),
                name: project.name.clone(),
                description: project.description.clone(),
                company_id: Uuid::from_slice(&project.company_id).unwrap(),
                project_status_id: Some(Uuid::from_slice(&project.project_status_id).unwrap()),
                created_at: project.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: project.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            });
        }
        Ok(ResProjectsOfInfoDto {
            total: res_projects.len() as u32,
            projects: res_projects
        })
        // if belong to user return all info of project
        
    }


}



    
