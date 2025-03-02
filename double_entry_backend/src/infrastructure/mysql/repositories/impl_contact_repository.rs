use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::{application::usecase::contact_usecase::ContactUseCaseError, domain::{dto::{contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto, ResUpdateContactDto}, std_201::ResCreateSuccess}, entities::{company, contact}, repository::require_implementation::trait_contact::ContactRepoReqImpl}};






pub struct ImplContactRepository {
    pub db: Arc<DatabaseConnection>
}



impl ImplContactRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplContactRepository {
            db
        }
    }
}

#[derive(Debug, Error)]
pub enum ContactRepositoryError {
    #[error("Contact not found")]
    ContactNotFound,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Duplicate contact")]
    UnAuthorized,
}

impl From<ContactRepositoryError> for ContactUseCaseError {
    fn from(error: ContactRepositoryError) -> Self {
        match error {
            ContactRepositoryError::ContactNotFound => ContactUseCaseError::ContactNotFound,
            ContactRepositoryError::InternalServerError => ContactUseCaseError::InternalServerError,
            ContactRepositoryError::UnAuthorized => ContactUseCaseError::UnAuthorized,
        }
    }
}

#[async_trait::async_trait]
impl ContactRepoReqImpl for ImplContactRepository{
    async fn create_contact(
        &self, 
        user_id: Uuid, 
        contact_data: ReqCreateContactDto
    ) 
    -> Result<ResCreateSuccess, ContactRepositoryError> {
        let mut new_contact = contact::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()),
            company_id: Set(contact_data.company_id.as_bytes().to_vec()),
            name: Set(contact_data.name.clone()),
            user_id: Set(user_id.as_bytes().to_vec()),
            ..Default::default()
        };
        if let Some(data) = contact_data.contact_type_id {
            new_contact.contact_type_id = Set(data.as_bytes().to_vec());
        }
        let result = contact::Entity::insert(new_contact).exec_with_returning(&*self.db).await;
        match result {
            Ok(data) => {
                Ok(ResCreateSuccess {
                    id_created: Uuid::from_slice(&data.id).unwrap()
                })
            },
            Err(_) => {
                Err(ContactRepositoryError::InternalServerError)
            }
        }
        
    }

    async fn get_contact(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) -> Result<ResEntryContactDto, ContactRepositoryError> {
        let result = contact::Entity::find_by_id(contact_id)
                .filter(company::Column::UserId.eq(user_id.as_bytes().to_vec()))
                .one(&*self.db)
                .await;
        if result.is_err() {
            return Err(ContactRepositoryError::InternalServerError);
        }
        if let Some(data) = result.unwrap() {
            Ok(ResEntryContactDto {
                id: Uuid::from_slice(&data.id).unwrap(),
                name: data.name.clone(),
                company_id: Uuid::from_slice(&data.company_id).unwrap(),
                user_id: Uuid::from_slice(&data.user_id).unwrap(),
                contact_type_id: Uuid::from_slice(&data.contact_type_id).unwrap(),
                created_at: data.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: data.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            })
        } else {
            Err(ContactRepositoryError::ContactNotFound)
        }

        
    }

    async fn get_contacts(
        &self, 
        user_id: Uuid
    ) -> Result<ResListContactDto, ContactRepositoryError> {
        let result = contact::Entity::find()
        .filter(contact::Column::UserId.eq(user_id.as_bytes().to_vec()))
        .all(&*self.db).await;
        if result.is_err() {
            return Err(ContactRepositoryError::InternalServerError);
        }
        let contacts = result.unwrap();
        let mut res_contacts = vec![];
        for contact in contacts {
            res_contacts.push(ResEntryContactDto {
                id: Uuid::from_slice(&contact.id).unwrap(),
                name: contact.name.clone(),
                company_id: Uuid::from_slice(&contact.company_id).unwrap(),
                user_id: Uuid::from_slice(&contact.user_id).unwrap(),
                contact_type_id: Uuid::from_slice(&contact.contact_type_id).unwrap(),
                created_at: contact.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: contact.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            });
        }
        Ok(ResListContactDto {
            total: res_contacts.len() as u32,
            contacts: res_contacts
        })
    }



    async fn update_contact(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid, 
        contact_data: ReqUpdateContactDto
    ) -> Result<ResUpdateContactDto, ContactRepositoryError> {

        let get_contact = contact::Entity::find_by_id(contact_id)
                .filter(contact::Column::UserId.eq(user_id.as_bytes().to_vec()))
                .one(&*self.db)
                .await;
        if get_contact.is_err() {
            return Err(ContactRepositoryError::InternalServerError);
        }
        match get_contact.unwrap() {
            Some(data) => {

                let mut before_update = data.into_active_model();

                if let Some(contact_type_id) = contact_data.contact_type_id {
                    before_update.contact_type_id = Set(contact_type_id.as_bytes().to_vec());
                }
                if let Some(name) = contact_data.name {
                    before_update.name = Set(name);
                }
                
                
                let result = before_update.update(&*self.db).await;
                if result.is_err() {
                    return Err(ContactRepositoryError::InternalServerError);
                }
                let updated_contact = result.unwrap();
                Ok(
                    ResUpdateContactDto {
                        id: Uuid::from_slice(&updated_contact.id).unwrap(),
                        name: updated_contact.name,
                        company_id: Uuid::from_slice(&updated_contact.company_id).unwrap(),
                        updated_at: updated_contact.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
                    }
                )
            },
            None => {
                Err(ContactRepositoryError::ContactNotFound)
            }
        }

        
    }

    async fn delete_contact(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) -> Result<(), ContactRepositoryError> {
        let get_contact = contact::Entity::find_by_id(contact_id)
                .filter(contact::Column::UserId.eq(user_id.as_bytes().to_vec()))
                .one(&*self.db)
                .await;
        if get_contact.is_err() {
            return Err(ContactRepositoryError::InternalServerError);
        }
        if let Some(data) = get_contact.unwrap() {
            let result = data.delete(&*self.db).await;
            if result.is_err() {
                return Err(ContactRepositoryError::InternalServerError);
            }
            Ok(())
        } else {
            Err(ContactRepositoryError::ContactNotFound)
        }
    }
}