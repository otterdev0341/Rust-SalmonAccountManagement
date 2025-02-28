
use rocket::{http::{ContentType, Status}, response::Responder, Request};
use serde::{Deserialize, Serialize};
use utoipa::openapi::Response;
use std::io::Cursor;

use super::api_response::ApiErrorResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiCreatedResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}


impl ApiCreatedResponse<String> {
    pub fn new(status: &str, message: &str, data: String) -> Self {
        ApiCreatedResponse {
            status: status.to_string(),
            message: message.to_string(),
            data,
        }
    }
}



impl<'r, T> Responder<'r, 'r> for ApiCreatedResponse<T>
where
    T: Serialize + 'r,  // Ensure that T is serializable and has the same or longer lifetime as 'r
{
    fn respond_to(self, _: &'r Request) -> rocket::response::Result<'r> {
        let body_string = serde_json::to_string(&self)
            .map_err(|_| Status::InternalServerError)?;  // Serialize the inner struct directly (not Json)
        let body_bytes = body_string.into_bytes();  // Convert the string to bytes
        let cursor = Cursor::new(body_bytes);  // Wrap bytes in a Cursor for AsyncRead

        rocket::Response::build()
            .status(Status::Created)
            .sized_body(cursor.get_ref().len(), cursor)  // Set the body and its length
            .header(ContentType::JSON)  // Set the content type to JSON
            .ok()
    }
}

pub type ApiCreatedResponseType<T> = Result<ApiCreatedResponse<T>, ApiErrorResponse>;