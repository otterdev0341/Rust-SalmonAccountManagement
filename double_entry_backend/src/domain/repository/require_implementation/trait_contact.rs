use sea_orm_migration::async_trait;
use uuid::Uuid;

use crate::{domain::dto::{contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto, ResUpdateContactDto}, std_201::ResCreateSuccess}, infrastructure::mysql::repositories::impl_contact_repository::ContactRepositoryError};





#[async_trait::async_trait]
pub trait ContactRepoReqImpl {
    async fn create_contact(&self, user_id: Uuid, contact_data: ReqCreateContactDto) -> Result<ResCreateSuccess, ContactRepositoryError>;
    async fn get_contact(&self, user_id: Uuid, contact_id: Uuid) -> Result<ResEntryContactDto, ContactRepositoryError>;
    async fn get_contacts(&self, user_id: Uuid) -> Result<ResListContactDto, ContactRepositoryError>;
    async fn update_contact(&self, user_id: Uuid, contact_id: Uuid, contact_data: ReqUpdateContactDto) -> Result<ResUpdateContactDto, ContactRepositoryError>;
    async fn delete_contact(&self, user_id: Uuid, contact_id: Uuid) -> Result<(), ContactRepositoryError>;
    
}