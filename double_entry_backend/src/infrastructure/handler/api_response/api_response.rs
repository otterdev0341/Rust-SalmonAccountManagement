use rocket::{response::Responder, Request};
use rocket::serde::Serialize;
use rocket::http::{Status, ContentType};
use serde::Deserialize;
use std::io::Cursor;
  // Import Cursor for AsyncRead

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSuccessResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub status: String,
    pub message: String,
}


impl ApiSuccessResponse<String> {
    pub fn new(status: &str,data: String) -> Self {
        ApiSuccessResponse {
            status: status.to_string(),
            data,
        }
    }
}


impl ApiErrorResponse {
    pub fn new(status: String,message: String) -> Self {
        ApiErrorResponse {
            status: status,
            message: message,
        }
    }
}


// Implementing Responder for ApiSuccessResponse
impl<'r, T> Responder<'r, 'r> for ApiSuccessResponse<T>
where
    T: Serialize + 'r,  // Ensure that T is serializable and has the same or longer lifetime as 'r
{
    fn respond_to(self, _: &'r Request) -> rocket::response::Result<'r> {
        let body_string = serde_json::to_string(&self).unwrap();  // Serialize the inner struct directly (not Json)
        let body_bytes = body_string.into_bytes();  // Convert the string to bytes
        let cursor = Cursor::new(body_bytes);  // Wrap bytes in a Cursor for AsyncRead

        rocket::Response::build()
            .status(Status::Ok)
            .sized_body(cursor.get_ref().len(), cursor)  // Set the body and its length
            .header(ContentType::JSON)  // Set the content type to JSON
            .ok()
    }
}

// Implementing Responder for ApiErrorResponse
impl<'r> Responder<'r, 'r> for ApiErrorResponse {
    fn respond_to(self, _: &'r Request) -> rocket::response::Result<'r> {
        let body_string = serde_json::to_string(&self).unwrap();  // Serialize the inner struct directly (not Json)
        let body_bytes = body_string.into_bytes();  // Convert the string to bytes
        let cursor = Cursor::new(body_bytes);  // Wrap bytes in a Cursor for AsyncRead

        rocket::Response::build()
            .status(Status::from_code(self.status.parse::<u16>().unwrap()).unwrap_or(Status::InternalServerError))
            .sized_body(cursor.get_ref().len(), cursor)  // Set the body and its length
            .header(ContentType::JSON)  // Set the content type to JSON
            .ok()
    }
}

pub type ApiResponse<T> = Result<ApiSuccessResponse<T>, ApiErrorResponse>;
