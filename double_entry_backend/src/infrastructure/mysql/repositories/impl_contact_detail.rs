use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::{application::usecase::contact_detail_usecase::ContactDetailUseCaseError, domain::{dto::{contact_detail_dto::{ReqCreateContactDetailDto, ReqUpdateContactDetailDto, ResEntryContactDetailDto, ResListContactDetailDto, ResUpdateContactDetailDto}, std_201::ResCreateSuccess}, entities::contact_detail, repository::require_implementation::trait_contact_detail::ContactDetailRepoReqImpl}};






pub struct ImplContactDetailRespository {
    pub db: Arc<DatabaseConnection>
}

impl ImplContactDetailRespository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplContactDetailRespository {
            db
        }
    }
}


#[derive(Debug,Error)]
pub enum ContactDetailRepositoryError{
    #[error("Contact detail not found")]
    ContactDetailNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Fail to delete contact detail")]
    DeleteFailed,
    #[error("Fail to update contact detail")]
    UpdateFailed,
}

impl From<ContactDetailRepositoryError> for ContactDetailUseCaseError {
    fn from(error: ContactDetailRepositoryError) -> Self {
        match error {
            ContactDetailRepositoryError::ContactDetailNotFound => ContactDetailUseCaseError::ContactDetailNotFound,
            ContactDetailRepositoryError::InternalServerError => ContactDetailUseCaseError::InternalServerError,
            ContactDetailRepositoryError::DeleteFailed => ContactDetailUseCaseError::DeleteFailed,
            ContactDetailRepositoryError::UpdateFailed => ContactDetailUseCaseError::UpdateFailed
        }
    }
}

#[async_trait::async_trait]
impl ContactDetailRepoReqImpl for ImplContactDetailRespository{






    
    async fn create_contact_detail(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid, 
        contact_detail_data: ReqCreateContactDetailDto
    ) -> Result<ResCreateSuccess, ContactDetailRepositoryError>{
        
        // create new contact detail
        let mut new_contact_detail = contact_detail::ActiveModel{
            id: Set(Uuid::new_v4().as_bytes().to_vec()),
            contact_id: Set(contact_id.as_bytes().to_vec()),
            user_id: Set(user_id.as_bytes().to_vec()),
            ..Default::default()
        };
        if let Some(mobile_1) = contact_detail_data.mobile_phone_1 {
            new_contact_detail.mobile_phone_1 = Set(mobile_1);
        }

        if let Some(mobile_2) = contact_detail_data.mobile_phone_2 {
            new_contact_detail.mobile_phone_2 = Set(mobile_2);
        }

        if let Some(mobile_3) = contact_detail_data.mobile_phone_3 {
            new_contact_detail.mobile_phone_3 = Set(mobile_3);
        }

        if let Some(email) = contact_detail_data.email {
            new_contact_detail.email = Set(email);
        }

        if let Some(address) = contact_detail_data.address {
            new_contact_detail.address = Set(address);
        }
        // try to save it
        let create_result = contact_detail::Entity::insert(new_contact_detail).exec_with_returning(&*self.db).await;

        //check is it saved or not
        if create_result.is_err(){
            return Err(ContactDetailRepositoryError::InternalServerError);
        }
        match create_result {
            Ok(data) => Ok(ResCreateSuccess {
                id_created: Uuid::from_slice(&data.id).unwrap()
            }),
            Err(_) => return Err(ContactDetailRepositoryError::InternalServerError)
        }
        
    }








    async fn get_contact_detail(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) -> Result<ResEntryContactDetailDto, ContactDetailRepositoryError>
    {
        let result = contact_detail::Entity::find_by_id(contact_id)
            .filter(contact_detail::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;

        if result.is_err() {
            return Err(ContactDetailRepositoryError::InternalServerError)
        }

        if let Some(data) = result.unwrap() {
            Ok(ResEntryContactDetailDto {
                id: Uuid::from_slice(&data.id).unwrap(),
                contact_id: Uuid::from_slice(&data.contact_id).unwrap(),
                user_id: Uuid::from_slice(&data.user_id).unwrap(),
                mobile_phone_1: data.mobile_phone_1,
                mobile_phone_2: data.mobile_phone_2,
                mobile_phone_3: data.mobile_phone_3,
                email: data.email,
                address: data.address,
                created_at: data.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: data.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            })
        } else {
            Err(ContactDetailRepositoryError::ContactDetailNotFound)
        }    
        
    }






    async fn get_contact_details(
        &self, 
        user_id: Uuid
    ) 
    -> Result<ResListContactDetailDto, ContactDetailRepositoryError>{
        // try to get all
        let result = contact_detail::Entity::find()
            .filter(contact_detail::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .all(&*self.db).await;

        // check if it is error
        if result.is_err() {
            return Err(ContactDetailRepositoryError::InternalServerError)
        }
        // check data the return
        let contact_details = result.unwrap();
        let mut res_contact_details = vec![];
        for contact_detail in contact_details {
            res_contact_details.push(ResEntryContactDetailDto {
                id: Uuid::from_slice(&contact_detail.id).unwrap(),
                contact_id: Uuid::from_slice(&contact_detail.contact_id).unwrap(),
                user_id: Uuid::from_slice(&contact_detail.user_id).unwrap(),
                mobile_phone_1: contact_detail.mobile_phone_1,
                mobile_phone_2: contact_detail.mobile_phone_2,
                mobile_phone_3: contact_detail.mobile_phone_3,
                email: contact_detail.email,
                address: contact_detail.address,
                created_at: contact_detail.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: contact_detail.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            });
        }
        Ok(ResListContactDetailDto {
            total: res_contact_details.len() as u32,
            contact_details: res_contact_details
        })  
        
        
    }






    async fn update_contact_detail(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid, 
        contact_detail_data: ReqUpdateContactDetailDto
    ) -> Result<ResUpdateContactDetailDto, ContactDetailRepositoryError>{
        let target_contact_detail = contact_detail::Entity::find_by_id(contact_id)
            .filter(contact_detail::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;

        if target_contact_detail.is_err() {
            return Err(ContactDetailRepositoryError::InternalServerError)
        }

        match target_contact_detail {
            Ok(data) => {

                if let Some(target_object) = data{
                    // extract to perfore update
                    let mut before_update = target_object.into_active_model();
                    // handle Option field data
                    if let Some(mobile_1) = contact_detail_data.mobile_phone_1 {
                        before_update.mobile_phone_1 = Set(mobile_1);
                    }
                    if let Some(mobile_2) = contact_detail_data.mobile_phone_2 {
                        before_update.mobile_phone_2 = Set(mobile_2);
                    }
                    if let Some(mobile_3) = contact_detail_data.mobile_phone_3 {
                        before_update.mobile_phone_3 = Set(mobile_3);
                    }
                    if let Some(email) = contact_detail_data.email {
                        before_update.email = Set(email);
                    }
                    if let Some(address) = contact_detail_data.address {
                        before_update.address = Set(address);
                    }
                    // try to update
                    let update_result = before_update.update(&*self.db).await;
                    if update_result.is_err() {
                        return Err(ContactDetailRepositoryError::UpdateFailed)
                    }
                    let updated_data = update_result.unwrap();
                    Ok(ResUpdateContactDetailDto {
                        id: Uuid::from_slice(&updated_data.id).unwrap(),
                        mobile_phone_1: updated_data.mobile_phone_1,
                        mobile_phone_2: updated_data.mobile_phone_2,
                        mobile_phone_3: updated_data.mobile_phone_3,
                        email: updated_data.email,
                        address: updated_data.address,
                        updated_at: updated_data.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
                    })
                    
                } else {
                    return Err(ContactDetailRepositoryError::ContactDetailNotFound)
                }            
            },
            Err(_) => Err(ContactDetailRepositoryError::InternalServerError)
        }
    }
    







    async fn delete_contact_detail(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) -> Result<(), ContactDetailRepositoryError>{
        // find target
        let target_contact_detail = contact_detail::Entity::find_by_id(contact_id)
            .filter(contact_detail::Column::UserId.eq(user_id.as_bytes().to_vec()))
            .one(&*self.db)
            .await;

        if target_contact_detail.is_err() {
            return Err(ContactDetailRepositoryError::InternalServerError)
        }

        match target_contact_detail {
            Ok(data) => {
                if let Some(target_object) = data {
                    let delete_result = target_object.delete(&*self.db).await;
                    if delete_result.is_err() {
                        return Err(ContactDetailRepositoryError::DeleteFailed)
                    }
                    Ok(())
                } else {
                    return Err(ContactDetailRepositoryError::ContactDetailNotFound)
                }
            },
            Err(_) => Err(ContactDetailRepositoryError::InternalServerError)
        }
    }

}