use rocket::http::Status;
use serde::Serialize;

use crate::infrastructure::handler::api_response::api_response::ApiErrorResponse;

use thiserror::Error;


#[derive(Error, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
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
        ApiErrorResponse {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}











#[derive(Error, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum SignInError {
    #[error("Invalid email")]
    InvalidEmail,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Email not found")]
    EmailNotFound,

    #[error("Incorrect email or password")]
    IncorrectEmailOrPassword,
}

impl From<SignInError> for ApiErrorResponse {
    fn from(err: SignInError) -> Self {
        let (status, message) = match err {
            SignInError::InvalidEmail => (Status::BadRequest, err.to_string()),
            SignInError::InvalidPassword => (Status::BadRequest, err.to_string()),
            SignInError::InternalServerError => (Status::InternalServerError, err.to_string()),
            SignInError::EmailNotFound => (Status::NotFound, err.to_string()),
            SignInError::IncorrectEmailOrPassword => (Status::Unauthorized, err.to_string()),
        };
        ApiErrorResponse {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}


#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token, not correct format")]
    InvalidToken,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Token not available")]
    TokenNotAvailable,
}

impl From<TokenError> for ApiErrorResponse {
    fn from(err: TokenError) -> Self {
        let (status, message) = match err {
            TokenError::TokenExpired => (Status::Unauthorized, err.to_string()),
            TokenError::InvalidToken => (Status::Unauthorized, err.to_string()),
            TokenError::InternalServerError => (Status::InternalServerError, err.to_string()),
            TokenError::TokenNotAvailable => (Status::NotFound, err.to_string()),
        };
        ApiErrorResponse {
            status: status.to_string(),
            message: message.to_string(),
        }
    }
}