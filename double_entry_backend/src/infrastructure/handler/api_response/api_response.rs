use core::fmt;

use rocket::{http::Status, Responder};
use sea_orm::DbErr;

#[derive(Responder)]
pub struct ApiSuccessResponse<T>(pub (Status, T));

#[derive(Responder, Debug)]
pub struct ApiErrorResponse(pub (Status, String));

pub type ApiResponse<T> = Result<ApiSuccessResponse<T>, ApiErrorResponse>;

impl From<DbErr> for ApiErrorResponse {
    fn from(err: DbErr) -> Self {
        ApiErrorResponse((Status::InternalServerError, format!("{:?}", err)))
    }
}

impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorResponse: {:?}, message = {}", (self.0).0, (self.0).1)
    }
}