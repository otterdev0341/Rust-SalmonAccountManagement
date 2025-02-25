use sea_orm_migration::async_trait;
use uuid::Uuid;
use crate::{domain::dto::{company_dto::{ReqCreateCompanyDto, ReqUpdateCompanyDto, ResEntryCompanyDto, ResListCompanyDto}, std_201::ResCreateSuccess}, infrastructure::mysql::repositories::impl_company_repository::CompanyRepositoryError};




#[async_trait::async_trait]
pub trait CompanyRepoReqImpl {
    async fn create_company(&self, user_id: Uuid,company_data: ReqCreateCompanyDto) -> Result<ResCreateSuccess, CompanyRepositoryError>;
    async fn get_company(&self ,user_id: Uuid,company_id: Uuid) -> Result<ResEntryCompanyDto, CompanyRepositoryError>;
    async fn get_companies(&self, user_id: Uuid) -> Result<ResListCompanyDto, CompanyRepositoryError>;
    async fn update_company(&self, user_id: Uuid, company_id: Uuid, company_data: ReqUpdateCompanyDto) -> Result<(), CompanyRepositoryError>;
    async fn delete_company(&self, user_id: Uuid, company_id: Uuid) -> Result<(), CompanyRepositoryError>;
}