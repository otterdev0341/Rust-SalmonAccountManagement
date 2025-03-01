use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::{dto::{company_dto::{ReqCreateCompanyDto, ReqUpdateCompanyDto, ResEntryCompanyDto, ResListCompanyDto, ResUpdateCompanyDto}, std_201::ResCreateSuccess}, repository::require_implementation::trait_company::CompanyRepoReqImpl};

pub struct CompanyUseCase<T>
where
    T: CompanyRepoReqImpl + Send + Sync,
{
    company_repo: Arc<T>,
}

#[derive(Debug, Error)]
pub enum CompanyUseCaseError {
    #[error("Company not found")]
    CompanyNotFound,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Duplicate company")]
    ConflictingCompany,

    #[error("Uuid convert error")]
    ConvertUuidError,

    #[error("Delete failed")]
    DeleteFailed,

    #[error("Update failed")]
    UpdateFailed,
}


impl<T> CompanyUseCase<T>
where
    T: CompanyRepoReqImpl + Send + Sync,
{
    pub fn new(company_repo: Arc<T>) -> Self {
        CompanyUseCase { company_repo }
    }

    pub async fn create_company(&self, user_id: Uuid,company_data: ReqCreateCompanyDto) -> Result<ResCreateSuccess, CompanyUseCaseError> {
        match self.company_repo.create_company(user_id, company_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_company(&self,user_id: Uuid, company_id: Uuid) -> Result<ResEntryCompanyDto, CompanyUseCaseError> {
        match self.company_repo.get_company(user_id, company_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn get_companies(&self, user_id: Uuid) -> Result<ResListCompanyDto, CompanyUseCaseError> {
        match self.company_repo.get_companies(user_id).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn update_company(&self, user_id: Uuid, company_id: Uuid, company_data: ReqUpdateCompanyDto) -> Result<ResUpdateCompanyDto, CompanyUseCaseError> {
        match self.company_repo.update_company(user_id, company_id, company_data).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e.into())
        }
    }

    pub async fn delete_company(&self, user_id: Uuid, company_id: Uuid) -> Result<(), CompanyUseCaseError> {
        match self.company_repo.delete_company(user_id, company_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

}