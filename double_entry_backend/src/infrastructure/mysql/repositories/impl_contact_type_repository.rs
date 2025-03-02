use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::{application::usecase::contact_type_usecase::ContactTypeUseCaseError, domain::{dto::{contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto, ResEntryContactTypeDto, ResListContactTypeDto, ResUpdateContactTypeDto}, std_201::ResCreateSuccess}, entities::contact_type, repository::require_implementation::trait_contact_type::ContactTypeRepoReqimpl}};




pub struct ImplContactTypeRepository {
    pub db: Arc<DatabaseConnection>
}

impl ImplContactTypeRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplContactTypeRepository {
            db
        }
    }
}



#[derive(Debug, Error)]
pub enum ContactTypeRepositoryError {
    #[error("Company not found")]
    CompanyNotFound,
    
    #[error("Internal server error")]
    InternalServerError,

    #[error("Delete failed")]
    DeleteFailed,

    #[error("Update failed")]
    UpdateFailed
}


impl From<ContactTypeRepositoryError> for ContactTypeUseCaseError {
    fn from(error: ContactTypeRepositoryError) -> Self {
        match error {
            ContactTypeRepositoryError::CompanyNotFound => ContactTypeUseCaseError::ContactTypeNotFound,
            ContactTypeRepositoryError::InternalServerError => ContactTypeUseCaseError::InternalServerError,         
            ContactTypeRepositoryError::DeleteFailed => ContactTypeUseCaseError::DeleteFailed,
            ContactTypeRepositoryError::UpdateFailed => ContactTypeUseCaseError::UpdateFailed
        }
    }
}


#[async_trait::async_trait]
impl ContactTypeRepoReqimpl for ImplContactTypeRepository {

    async fn create_contact_type(
        &self, 
        user_id: Uuid, 
        contact_type_data: ReqCreateContactTypeDto
    ) 
    -> Result<ResCreateSuccess, ContactTypeRepositoryError>{
            let new_contact_type = contact_type::ActiveModel {
                id: Set(Uuid::new_v4().as_bytes().to_vec()),
                name: Set(contact_type_data.name.clone()),
                user_id: Set(user_id.as_bytes().to_vec()),
                ..Default::default()
            };
            let insert_result = contact_type::Entity::insert(new_contact_type).exec_with_returning(&*self.db).await;
            match insert_result {
                Ok(data) => {
                    let res = ResCreateSuccess {
                        id_created: Uuid::from_slice(&data.id).unwrap()
                    };
                    Ok(res)
                },
                Err(_) => Err(ContactTypeRepositoryError::InternalServerError)
            }

            
    }
    
    async fn get_contact_type(
        &self, 
        user_id: Uuid, 
        contact_type_id: Uuid
    ) 
    -> Result<ResEntryContactTypeDto, ContactTypeRepositoryError>{
            let find_result = 
                    contact_type::Entity::find_by_id(contact_type_id)
                    .filter(contact_type::Column::UserId.eq(user_id.as_bytes().to_vec()))
                    .one(&*self.db)
                    .await;
            if find_result.is_err() {
                return Err(ContactTypeRepositoryError::CompanyNotFound);
            }
            if let Some(data) = find_result.unwrap() {
                let res = ResEntryContactTypeDto {
                    id: Uuid::from_slice(&data.id).unwrap(),
                    name: data.name.clone(),
                    description: data.description.clone(),
                    user_id: Uuid::from_slice(&data.user_id).unwrap(),
                    created_at: data.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                    updated_at: data.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
                };
                return Ok(res);
            }else {
                return Err(ContactTypeRepositoryError::CompanyNotFound);
            }

            
    }
    
    async fn get_contact_types(
        &self, 
        user_id: Uuid
    )
    -> Result<ResListContactTypeDto, ContactTypeRepositoryError>{
            let find_result = 
                    contact_type::Entity::find()
                    .filter(contact_type::Column::UserId.eq(user_id.as_bytes().to_vec()))
                    .all(&*self.db)
                    .await;
            if find_result.is_err() {
                return Err(ContactTypeRepositoryError::InternalServerError);
            }
            let data = find_result.unwrap();
            let mut res_data = vec![];
            for item in data {
                let res = ResEntryContactTypeDto {
                    id: Uuid::from_slice(&item.id).unwrap(),
                    name: item.name.clone(),
                    description: item.description.clone(),
                    user_id: Uuid::from_slice(&item.user_id).unwrap(),
                    created_at: item.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                    updated_at: item.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
                };
                res_data.push(res);
            }
            let res = ResListContactTypeDto {
                total: res_data.len() as u32,
                contact_types: res_data
                
            };
            Ok(res)
    }
    
    async fn update_contact_type(
        &self, 
        user_id: Uuid, 
        contact_type_id: Uuid, 
        contact_type_data: ReqUpdateContactTypeDto
    )
    -> Result<ResUpdateContactTypeDto, ContactTypeRepositoryError>{
    // let find it
    let find_result = 
            contact_type::Entity::find_by_id(contact_type_id)
            .filter(contact_type::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;
    // if erro on finding process
    if find_result.is_err() {
        return Err(ContactTypeRepositoryError::CompanyNotFound);
    }
    //if found
    if let Some(data) = find_result.unwrap() {
        // change to active model
        let mut updated_data = data.into_active_model();
        
        // handle option data
        if let Some(name) = contact_type_data.name {
            updated_data.name = Set(name);
        }
        if let Some(description) = contact_type_data.description {
            updated_data.description = Set(description);
        }
        // update the data
        let update_result = updated_data.update(&*self.db).await;
        
        // check is it error
        if update_result.is_err() {
            return Err(ContactTypeRepositoryError::UpdateFailed);
        }

        // extract
        let updated_data = update_result.unwrap();
        Ok(ResUpdateContactTypeDto {
            id: Uuid::from_slice(&updated_data.id).unwrap(),
            name: updated_data.name.clone(),
            description: updated_data.description.clone(),
            user_id: Uuid::from_slice(&updated_data.user_id).unwrap(),
            updated_at: updated_data.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
        })

    }else {
        return Err(ContactTypeRepositoryError::UpdateFailed);
    }
        

    }


    async fn delete_contact_type(
        &self, 
        user_id: Uuid, 
        contact_type_id: Uuid
    ) 
    -> Result<(), ContactTypeRepositoryError>{
        // find the data
        let find_result = 
                contact_type::Entity::find_by_id(contact_type_id)
                .filter(contact_type::Column::UserId.eq(user_id.as_bytes().to_vec()))
                .one(&*self.db)
                .await;
        // if error on finding process
        if find_result.is_err() {
            return Err(ContactTypeRepositoryError::CompanyNotFound);
        }
        // if found
        if let Some(data) = find_result.unwrap() {
            // delete the data
            let delete_result = data.delete(&*self.db).await;
            // check is it error
            if delete_result.is_err() {
                return Err(ContactTypeRepositoryError::DeleteFailed);
            } else {
                Ok(())
            }
        }else {
            return Err(ContactTypeRepositoryError::CompanyNotFound);
        }


    }
}
    
    

