use sea_orm_migration::async_trait;
use thiserror::Error;

use crate::domain::dto::company_dto::{ReqCreateCompanyDto, ReqUpdateCompanyDto, ResEntryCompanyDto};

#[derive(Debug,Error)]
pub enum CompanyRepositoryError {
    #[error("Company not found")]
    NotFound,
    #[error("Company already exists")]
    InternalServerError,
}


#[async_trait::async_trait]
pub trait CompanyRepoReqImpl {
    async fn create_company(&self, company_data: ReqCreateCompanyDto) -> Result<(), CompanyRepositoryError>;
    async fn get_company(&self, company_id: i32) -> Result<ResEntryCompanyDto, CompanyRepositoryError>;
    async fn get_companies(&self) -> Result<Vec<ResEntryCompanyDto>, CompanyRepositoryError>;
    async fn update_company(&self, company_id: i32, company_data: ReqUpdateCompanyDto) -> Result<(), CompanyRepositoryError>;
    async fn delete_company(&self, company_id: i32) -> Result<(), CompanyRepositoryError>;
}