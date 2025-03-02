
use sea_orm_migration::async_trait;
use uuid::Uuid;

use crate::{domain::dto::{contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto, ResEntryContactTypeDto, ResListContactTypeDto, ResUpdateContactTypeDto}, std_201::ResCreateSuccess}, infrastructure::mysql::repositories::impl_contact_type_repository::ContactTypeRepositoryError};



#[async_trait::async_trait]
pub trait ContactTypeRepoReqimpl {
    async fn create_contact_type(&self, user_id: Uuid, contact_type_data: ReqCreateContactTypeDto) 
        -> Result<ResCreateSuccess, ContactTypeRepositoryError>;
    
    async fn get_contact_type(&self, user_id: Uuid, contact_type_id: Uuid) 
        -> Result<ResEntryContactTypeDto, ContactTypeRepositoryError>;
    
    async fn get_contact_types(&self, user_id: Uuid) 
        -> Result<ResListContactTypeDto, ContactTypeRepositoryError>;
    
    async fn update_contact_type(&self, user_id: Uuid, contact_type_id: Uuid, contact_type_data: ReqUpdateContactTypeDto) 
        -> Result<ResUpdateContactTypeDto, ContactTypeRepositoryError>;
    
    async fn delete_contact_type(&self, user_id: Uuid, contact_type_id: Uuid) 
        -> Result<(), ContactTypeRepositoryError>;
}