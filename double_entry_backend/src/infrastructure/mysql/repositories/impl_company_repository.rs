use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::{application::usecase::company_usecase::CompanyUseCaseError, domain::{dto::{company_dto::{ReqCreateCompanyDto, ReqUpdateCompanyDto, ResEntryCompanyDto, ResListCompanyDto}, std_201::ResCreateSuccess}, entities::company, repository::require_implementation::trait_company::CompanyRepoReqImpl}};

pub struct ImplCompanyRespository {
    pub db: Arc<DatabaseConnection>
}


impl ImplCompanyRespository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplCompanyRespository {
            db
        }
    }
}


#[derive(Debug,Error)]
pub enum CompanyRepositoryError {
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
    UpdateFailed
}

impl From<CompanyRepositoryError> for CompanyUseCaseError {
    fn from(error: CompanyRepositoryError) -> Self {
        match error {
            CompanyRepositoryError::CompanyNotFound => CompanyUseCaseError::CompanyNotFound,
            CompanyRepositoryError::InternalServerError => CompanyUseCaseError::InternalServerError,
            CompanyRepositoryError::ConflictingCompany => CompanyUseCaseError::ConflictingCompany,
            CompanyRepositoryError::ConvertUuidError => CompanyUseCaseError::ConvertUuidError,
            CompanyRepositoryError::DeleteFailed => CompanyUseCaseError::DeleteFailed,
            CompanyRepositoryError::UpdateFailed => CompanyUseCaseError::UpdateFailed
        }
    }
}


#[async_trait::async_trait]
impl CompanyRepoReqImpl for ImplCompanyRespository {
    async fn create_company(&self, user_id: Uuid ,company_data: ReqCreateCompanyDto) -> Result<ResCreateSuccess, CompanyRepositoryError> {
        // Implement the logic to create a company
        
        let new_company = company::ActiveModel {
            id: Set(Uuid::new_v4().as_bytes().to_vec()),
            name: Set(company_data.name),
            description: Set(company_data.description),
            user_id: Set(user_id.as_bytes().to_vec()),
            ..Default::default()
        };
        let persist_result = company::Entity::insert(new_company).exec_with_returning(&*self.db).await;
        match persist_result {
            Ok(data) => Ok(ResCreateSuccess {
                id_created: Uuid::from_slice(&data.id).unwrap()
            }),
            Err(_) => return Err(CompanyRepositoryError::InternalServerError)
        }
    }

    async fn get_company(&self, user_id: Uuid ,company_id: Uuid) -> Result<ResEntryCompanyDto, CompanyRepositoryError> {
        // find the company
        let result = company::Entity::find_by_id(company_id)
                .filter(company::Column::UserId.eq(user_id.as_bytes().to_vec()))
                .one(&*self.db)
                .await;
        
        // if error return internal server error
        if result.is_err() {
            return Err(CompanyRepositoryError::InternalServerError)
        }

        // if fond to the operation
        if let Some(company) = result.unwrap() {
            
            // craete return dto
            Ok(ResEntryCompanyDto {
                id: Uuid::from_slice(&company.id).unwrap(),
                name: company.name,
                description: company.description,
                created_at: company.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: company.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            })
        // if not found return company not found
        } else {
            Err(CompanyRepositoryError::CompanyNotFound)
        }
    }

    async fn get_companies(&self, user_id: Uuid) -> Result<ResListCompanyDto, CompanyRepositoryError> {
        // Implement the logic to get all companies
        let result = company::Entity::find()
        .filter(company::Column::UserId.eq(user_id.as_bytes().to_vec()))
        .all(&*self.db).await;
        if result.is_err() {
            return Err(CompanyRepositoryError::InternalServerError)
        }

        let companies = result.unwrap();
        let mut res_companies = vec![];
        for company in companies {
            
            res_companies.push(ResEntryCompanyDto {
                id: Uuid::from_slice(&company.id).unwrap(),
                name: company.name,
                description: company.description,
                created_at: company.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: company.updated_at.map(|dt| dt.to_string()).unwrap_or_default()
            });
        }
        Ok(ResListCompanyDto {
            total: res_companies.len() as u32,
            companies: res_companies
        })
    }

    async fn update_company(&self, user_id: Uuid, company_id: Uuid, company_data: ReqUpdateCompanyDto) -> Result<(), CompanyRepositoryError> {
        // Implement the logic to update a company by id
        // check is this id exist
        
        let get_company = company::Entity::find_by_id(company_id)
                        .filter(company::Column::UserId.eq(user_id))
                        .one(&*self.db).await;
        // if query error return internal server error
        // If query error or company not found, return an error
        match get_company {
            Ok(Some(company)) => {
                // Prepare for update
                let mut before_update = company.into_active_model();

                // Set values conditionally if provided
                if let Some(name) = company_data.name {
                    before_update.name = Set(name);
                }
                if let Some(description) = company_data.description {
                    before_update.description = Set(description);
                }

                // Perform the update and check for failure
                match before_update.save(&*self.db).await {
                    Ok(_) => Ok(()),  // Update successful
                    Err(_) => Err(CompanyRepositoryError::UpdateFailed),  // Update failed
                }
            },
            Ok(None) => Err(CompanyRepositoryError::CompanyNotFound),  // Company not found
            Err(_) => Err(CompanyRepositoryError::InternalServerError),  // Query error
        }
        
    }

    async fn delete_company(&self, user_id: Uuid, company_id: Uuid) -> Result<(), CompanyRepositoryError> {
        // Implement the logic to delete a company by id
        let target 
                = company::Entity::find_by_id(company_id)
                    .filter(company::Column::UserId.eq(user_id))
                    .one(&*self.db).await;
        // if query error return internal server error
        if target.is_err() {
            return Err(CompanyRepositoryError::InternalServerError)
        }
        // if found the company try to delete
        if let Some(company) = target.unwrap() {
            let delete_result = company.delete(&*self.db).await;
            // check delete result
            if delete_result.is_err() {
                return Err(CompanyRepositoryError::DeleteFailed)
            } else {
                Ok(())
            }
        // if not found return company not found
        } else {
            return Err(CompanyRepositoryError::CompanyNotFound)
        }
    }
}