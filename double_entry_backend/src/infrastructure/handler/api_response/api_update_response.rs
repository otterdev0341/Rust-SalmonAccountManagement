
use rocket::{http::{ContentType, Status}, response::Responder, Request};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use super::api_response::ApiErrorResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiUpdateResponse<T>{
    pub status: String,
    pub message: String,
    pub data: T,
}

impl<T> ApiUpdateResponse<T> {
    pub fn new(status: &str, message: &str, data: T) -> Self {
        ApiUpdateResponse {
            status: status.to_string(),
            message: message.to_string(),
            data,
        }
    }
}


impl<'r, T> Responder<'r, 'r> for ApiUpdateResponse<T>
where
    T: Serialize + 'r,  // Ensure that T is serializable and has the same or longer lifetime as 'r
{
    fn respond_to(self, _: &'r Request) -> rocket::response::Result<'r> {
        let body_string = serde_json::to_string(&self)
            .map_err(|_| Status::InternalServerError)?;  // Serialize the inner struct directly (not Json)
        let body_bytes = body_string.into_bytes();  // Convert the string to bytes
        let cursor = Cursor::new(body_bytes);  // Wrap bytes in a Cursor for AsyncRead

        rocket::Response::build()
            .status(Status::Ok)
            .sized_body(cursor.get_ref().len(), cursor)  // Set the body and its length
            .header(ContentType::JSON)  // Set the content type to JSON
            .ok()
    }
}

pub type ApiUpdateResponseType<T> = Result<ApiUpdateResponse<T>, ApiErrorResponse>;