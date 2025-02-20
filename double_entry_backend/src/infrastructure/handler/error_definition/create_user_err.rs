use rocket::http::Status;

use crate::infrastructure::handler::api_response::api_response::ApiErrorResponse;

use thiserror::Error;


#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("Username already exists")]
    UsernameAlreadyExists,
    
    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Invalid username")]
    InvalidUsername,

    #[error("Invalid email")]
    InvalidEmail,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Invalid first name")]
    InvalidFirstName,

    #[error("Invalid last name")]
    InvalidLastName,

    #[error("Internal server error")]
    InternalServerError,
}

impl From<CreateUserError> for ApiErrorResponse {
    fn from(err: CreateUserError) -> Self {
        let (status, message) = match err {
            CreateUserError::UsernameAlreadyExists => (Status::Conflict, err.to_string()),
            CreateUserError::EmailAlreadyExists => (Status::Conflict, err.to_string()),
            CreateUserError::InvalidUsername => (Status::BadRequest, err.to_string()),
            CreateUserError::InvalidEmail => (Status::BadRequest, err.to_string()),
            CreateUserError::InvalidPassword => (Status::BadRequest, err.to_string()),
            CreateUserError::InvalidFirstName => (Status::BadRequest, err.to_string()),
            CreateUserError::InvalidLastName => (Status::BadRequest, err.to_string()),
            CreateUserError::InternalServerError => (Status::InternalServerError, err.to_string()),
        };
        ApiErrorResponse((status, message))
    }
}
