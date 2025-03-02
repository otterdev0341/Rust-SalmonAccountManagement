use sea_orm_migration::async_trait;
use uuid::Uuid;

use crate::{domain::dto::{contact_detail_dto::{ReqCreateContactDetailDto, ReqUpdateContactDetailDto, ResEntryContactDetailDto, ResListContactDetailDto, ResUpdateContactDetailDto}, std_201::ResCreateSuccess}, infrastructure::mysql::repositories::impl_contact_detail::ContactDetailRepositoryError};



#[async_trait::async_trait]
pub trait ContactDetailRepoReqImpl {

    async fn create_contact_detail(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid, 
        contact_detail_data: ReqCreateContactDetailDto
    ) -> Result<ResCreateSuccess, ContactDetailRepositoryError>;

    async fn get_contact_detail(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) -> Result<ResEntryContactDetailDto, ContactDetailRepositoryError>;

    async fn get_contact_details(
        &self, 
        user_id: Uuid
    ) -> Result<ResListContactDetailDto, ContactDetailRepositoryError>;

    async fn update_contact_detail(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid, 
        contact_detail_data: ReqUpdateContactDetailDto
    ) -> Result<ResUpdateContactDetailDto, ContactDetailRepositoryError>;
    
    async fn delete_contact_detail(
        &self, 
        user_id: Uuid, 
        contact_id: Uuid
    ) -> Result<(), ContactDetailRepositoryError>;
    
}