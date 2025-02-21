use rocket::http::Status;
use rocket::response::{Responder, Result};
use rocket::Request;
use thiserror::Error;

use crate::infrastructure::handler::api_response::api_response::{ApiErrorResponse, ApiSuccessResponse, SuccessMessage};


#[derive(Error, Debug)]
pub enum CompanyError {
    #[error("Invalid company ID or user ID")]
    InvalidId,

    #[error("Forbidden: You don't have permission to perform this action")]
    Forbidden,

    #[error("Company or user not found")]
    NotFound,

    #[error("User already added to this company")]
    UserAlreadyAdded,

    #[error("User already removed from this company")]
    UserAlreadyRemoved,

    #[error("Company creation failed due to invalid name or description")]
    InvalidCompanyData,

    #[error("Internal server error")]
    InternalError,
    
}

impl From<CompanyError> for ApiErrorResponse {
    fn from(err: CompanyError) -> Self {
        let (status, message) = match err {
            CompanyError::InvalidId => (Status::BadRequest, err.to_string()),
            CompanyError::Forbidden => (Status::Forbidden, err.to_string()),
            CompanyError::NotFound => (Status::NotFound, err.to_string()),
            CompanyError::UserAlreadyAdded => (Status::Conflict, err.to_string()),
            CompanyError::UserAlreadyRemoved => (Status::Conflict, err.to_string()),
            CompanyError::InvalidCompanyData => (Status::BadRequest, err.to_string()),
            CompanyError::InternalError => (Status::InternalServerError, err.to_string()),
        };
        ApiErrorResponse((status, message))
    }
}



#[derive(Error, Debug)]
pub enum CompanySuccess {
    #[error("User added to company")]
    UserAdded,

    #[error("User removed from company")]
    UserRemoved,

    #[error("Company created successfully")]
    CompanyCreated,
    #[error("Company updated successfully")]
    UpdateSuccess,
    #[error("Company deleted successfully")]
    DeleteSuccess,
}

impl From<CompanySuccess> for ApiSuccessResponse<SuccessMessage> {
    fn from(err: CompanySuccess) -> Self {
        let (status, message) = match err {
            CompanySuccess::UserAdded => (Status::Created, err.to_string()),
            CompanySuccess::UserRemoved => (Status::Ok, err.to_string()),
            CompanySuccess::CompanyCreated => (Status::Ok, err.to_string()),
            CompanySuccess::UpdateSuccess => (Status::Ok, err.to_string()),
            CompanySuccess::DeleteSuccess => (Status::Ok, err.to_string()),
        };
        ApiSuccessResponse((status, SuccessMessage { message: message.to_string() }))
    }
}

