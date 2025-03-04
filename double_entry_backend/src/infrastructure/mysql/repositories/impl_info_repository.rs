use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::{application::usecase::info_usecase::InfoUseCaseError, domain::{dto::{info_dto::{ReqCreateInfoDto, ReqUpdateInfoDto, ResEntryInfoDto, ResListInfoDto, ResUpdateInfoDto}, std_201::ResCreateSuccess}, entities::info, repository::require_implementation::trait_info::{InfoRepoReqImpl, InfoRepositoryError}}};

pub struct ImplInfoRepository{
    pub db: Arc<DatabaseConnection>
}


impl ImplInfoRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplInfoRepository {
            db
        }
    }
}



impl From<InfoRepositoryError> for InfoUseCaseError {
    fn from(error: InfoRepositoryError) -> Self {
        match error {
            InfoRepositoryError::InfoNotFound => InfoUseCaseError::InfoNotFound,
            InfoRepositoryError::InternalServerError => InfoUseCaseError::InternalServerError,
            InfoRepositoryError::DeleteFailed => InfoUseCaseError::DeleteFailed,
            InfoRepositoryError::UpdateFailed => InfoUseCaseError::UpdateFailed,
            
                    }
    }
}

#[async_trait::async_trait]
impl InfoRepoReqImpl for ImplInfoRepository {
    
    async fn create_info(
        &self, 
        user_id: Uuid, 
        info_data: ReqCreateInfoDto
    ) -> Result<ResCreateSuccess, InfoRepositoryError>
    {
        let new_info = info::ActiveModel {
            user_id: Set(user_id.as_bytes().to_vec()),
            title: Set(info_data.title),
            content: Set(info_data.content),
            ..Default::default()
        };
        let inserted_result = info::Entity::insert(new_info).exec_with_returning(&*self.db).await;
        match inserted_result {
            Ok(data) => Ok(ResCreateSuccess {
                id_created: Uuid::from_slice(&data.id).unwrap()
            }),
            Err(_) => Err(InfoRepositoryError::InternalServerError)
        }
    }

    async fn get_info(
        &self, 
        user_id: Uuid,
        info_id: Uuid
    ) -> Result<ResEntryInfoDto, InfoRepositoryError>
    {
        let target_info = info::Entity::find_by_id(info_id.as_bytes().to_vec())
            .filter(info::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        if target_info.is_err() {
            return Err(InfoRepositoryError::InternalServerError);
        }
        match target_info.unwrap() {
            Some(data) => Ok(ResEntryInfoDto {
                id: Uuid::from_slice(&data.id).unwrap(),
                user_id: Uuid::from_slice(&data.user_id).unwrap(),
                title: data.title.clone(),
                content: data.content.clone(),
                created_at: data.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: data.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            }),
            None => Err(InfoRepositoryError::InfoNotFound)
        }
    }

    async fn get_infos(
        &self, 
        user_id: Uuid
    ) -> Result<ResListInfoDto, InfoRepositoryError>
    {
        let result = info::Entity::find()
            .filter(info::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .all(&*self.db)
            .await;
        if result.is_err() {
            return Err(InfoRepositoryError::InternalServerError);
        }
        let infos = result.unwrap();
        let mut res_infos = vec![];
        for info in infos {
            res_infos.push(ResEntryInfoDto {
                id: Uuid::from_slice(&info.id).unwrap(),
                user_id: Uuid::from_slice(&info.user_id).unwrap(),
                title: info.title.clone(),
                content: info.content.clone(),
                created_at: info.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: info.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            });
        }
        Ok(ResListInfoDto {
            total: res_infos.len() as u32,
            infos: res_infos
        })
    }
    
    async fn update_info(
        &self, 
        user_id: Uuid,
        info_id: Uuid, 
        info_data: ReqUpdateInfoDto
    ) -> Result<ResUpdateInfoDto, InfoRepositoryError>
    {
        let is_avaliable = info::Entity::find_by_id(info_id.as_bytes().to_vec())
            .filter(info::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        if is_avaliable.is_err() {
            return Err(InfoRepositoryError::InternalServerError);
        }

        match is_avaliable.unwrap() {
            Some(data) => {
                let mut prepare_to_update = data.into_active_model();
                if let Some(title) = info_data.title {
                    prepare_to_update.title = Set(title);
                }
                if let Some(content) = info_data.content {
                    prepare_to_update.content = Set(content);
                }
                let updated_result = prepare_to_update.update(&*self.db).await;
                if updated_result.is_err() {
                    return Err(InfoRepositoryError::UpdateFailed);
                }
                let updated_data = updated_result.unwrap();
                Ok(ResUpdateInfoDto {
                    id: Uuid::from_slice(&updated_data.id).unwrap(),
                    user_id: Uuid::from_slice(&updated_data.user_id).unwrap(),
                    title: updated_data.title.clone(),
                    content: updated_data.content.clone(),
                    updated_at: updated_data.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
                })
            },
            None => Err(InfoRepositoryError::InfoNotFound)
        }

        
    }

    async fn delete_info(
        &self, 
        user_id: Uuid,
        info_id: Uuid
    ) -> Result<(), InfoRepositoryError>
    {
        let target = info::Entity::find_by_id(info_id.as_bytes().to_vec())
            .filter(info::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
        if target.is_err() {
            return Err(InfoRepositoryError::InternalServerError);
        }
        match target.unwrap() {
            Some(data) => {
                let delete_result = data.delete(&*self.db).await;
                if delete_result.is_err() {
                    return Err(InfoRepositoryError::DeleteFailed);
                }
                Ok(())
            },
            None => Err(InfoRepositoryError::InfoNotFound)
        }
    }
}