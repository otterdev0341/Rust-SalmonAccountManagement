// pub struct ReqCreateUserDto {

//     #[validate(length(min = 3, max = 15, message = "username must be between 3 and 15 characters"))]
//     pub username: String,

//     #[validate(length(min = 3, max = 20, message = "first name must be between 3 and 20 characters"))]
//     pub first_name: String,

//     #[validate(length(min = 3, max = 20, message = "last name must be between 3 and 20 characters"))]
//     pub last_name: String,

//     #[validate(email)]
//     pub email: String,
    
//     #[validate(length(min = 6, max = 15, message = "password must be at least 6 characters to 15 characters"))]
//     pub password: String,
// }

use thiserror::Error;

use crate::domain::dto::auth_dto::ReqCreateUserDto;

#[derive(Debug, Error)]
pub enum ValidateReqCreateUserDtoError{
    #[error("Empty String not allowed")]
    EmptyString,
    #[error("Invalid Length of data")]
    LengthError,
    #[error("Invalid Email")]
    InvalidEmail,
    
}

pub fn check_req_create_user_dto(req_create_user_dto: &ReqCreateUserDto) -> Result<(), ValidateReqCreateUserDtoError> {
    if req_create_user_dto.username.is_empty() {
        return Err(ValidateReqCreateUserDtoError::EmptyString);
    }
    if req_create_user_dto.first_name.is_empty() {
        return Err(ValidateReqCreateUserDtoError::EmptyString);
    }
    if req_create_user_dto.last_name.is_empty() {
        return Err(ValidateReqCreateUserDtoError::EmptyString);
    }
    if req_create_user_dto.email.is_empty() {
        return Err(ValidateReqCreateUserDtoError::EmptyString);
    }
    if req_create_user_dto.password.is_empty() {
        return Err(ValidateReqCreateUserDtoError::EmptyString);
    }
    if req_create_user_dto.username.len() < 3 || req_create_user_dto.username.len() > 15 {
        return Err(ValidateReqCreateUserDtoError::LengthError);
    }
    if req_create_user_dto.first_name.len() < 3 || req_create_user_dto.first_name.len() > 20 {
        return Err(ValidateReqCreateUserDtoError::LengthError);
    }
    if req_create_user_dto.last_name.len() < 3 || req_create_user_dto.last_name.len() > 20 {
        return Err(ValidateReqCreateUserDtoError::LengthError);
    }
    if req_create_user_dto.password.len() < 6 || req_create_user_dto.password.len() > 15 {
        return Err(ValidateReqCreateUserDtoError::LengthError);
    }
    if !req_create_user_dto.email.contains("@") {
        return Err(ValidateReqCreateUserDtoError::InvalidEmail);
    }
    Ok(())
}
