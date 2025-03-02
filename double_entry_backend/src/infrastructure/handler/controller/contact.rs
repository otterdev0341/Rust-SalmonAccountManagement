use std::sync::Arc;

use rocket::{delete, get, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;
use crate::{application::usecase::contact_usecase::{ContactUseCase, ContactUseCaseError}, domain::{dto::{auth_dto::AuthenticatedUser, contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto, ResUpdateContactDto}, std_201::ResCreateSuccess}, entities::contact}, infrastructure::{faring::cors::options, handler::api_response::{api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, api_success_response::{ApiCreatedResponse, ApiCreatedResponseType}}, mysql::repositories::impl_contact_repository::ImplContactRepository}};


pub fn contact_routes() -> Vec<Route> {
    routes![
        view_contact,
        view_contacts,
        create_contact,
        edit_contact,
        delete_contact,
        options
    ]
}

impl From<ContactUseCaseError> for ApiErrorResponse {
    fn from(error: ContactUseCaseError) -> Self {
        match error {
            ContactUseCaseError::ContactNotFound => ApiErrorResponse::new(404, "Contact not found".to_string()),
            ContactUseCaseError::InternalServerError => ApiErrorResponse::new(500, "Internal server error".to_string()),
            ContactUseCaseError::UnAuthorized => ApiErrorResponse::new(401, "UnAuthorized".to_string())
        }
    }
}


#[utoipa::path(
    get,
    path = "/contact/<contact_id>",
    summary = "View contact",
    description = "View contact",
    tags = ["contact"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "contact found",
            body = ResEntryContactDto,
            description = "contact data"
    ),
        (status = 400, description = "Invalid contact"),
        (status = 404, description = "contact not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/<contact_id>")]
pub async fn view_contact(
    user: AuthenticatedUser,
    contact_id : String,
    contact_usecase: &State<Arc<ContactUseCase<ImplContactRepository>>>
) 
-> ApiResponse<ResEntryContactDto> {
    
    let operaion = 
        contact_usecase
        .get_contact(user.id, Uuid::parse_str(&contact_id).unwrap()).await;
    
    match operaion {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(e) => {
            return Err(e.into())
        }
    }
}




#[utoipa::path(
    get,
    path = "contact/",
    summary = "View contacts",
    description = "View contacts",
    tags = ["contact"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "contacts found",
            body = ResListContactDto,
            description = "contacts data"
    ),
        (status = 400, description = "Invalid contacts"),
        (status = 404, description = "Customers not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/")]
pub async fn view_contacts(
    user: AuthenticatedUser,
    contact_usecase: &State<Arc<ContactUseCase<ImplContactRepository>>>
) 
-> ApiResponse<ResListContactDto> {
    let operaion = contact_usecase.get_contacts(user.id).await;
    match operaion {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(e) => {
            return Err(e.into())
        }
    }
    
}





#[utoipa::path(
    post,
    path = "/contact",
    summary = "Create contact",
    request_body = ReqCreateContactDto,
    description = "Create contact",
    tags = ["contact"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "contact created"),
        (status = 400, description = "Invalid contact"),
        (status = 404, description = "contact not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[post("/", format = "json", data = "<contact_id>")]
pub async fn create_contact(
    user: AuthenticatedUser,
    contact_id : Json<ReqCreateContactDto>,
    contact_usecase: &State<Arc<ContactUseCase<ImplContactRepository>>>
) 
-> ApiCreatedResponseType<ResCreateSuccess> 
{
    let operation = contact_usecase.create_contact(user.id, contact_id.into_inner()).await;
    match operation {
        Ok(data) => {
            return Ok(ApiCreatedResponse{
                status: "success".to_string(),
                message: "Contact created".to_string(),
                data: data
            })
        },
        Err(e) => {
            return Err(e.into())
        }
    }
}





#[utoipa::path(
    put,
    path = "/contact/{contact_id}",
    summary = "Edit contact",
    request_body = ReqUpdateContactDto,
    description = "Edit contact",
    tags = ["contact"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_id" = String,Path, description = "contact ID")
    ),
    responses(
        (status = 200, description = "contact edited"),
        (status = 400, description = "Invalid contact"),
        (status = 404, description = "contact not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[put("/<contact_id>", format = "json", data = "<contact_data>")]
pub async fn edit_contact(
    user: AuthenticatedUser,
    contact_id : String,
    contact_data : Json<ReqUpdateContactDto>,
    contact_usecase: &State<Arc<ContactUseCase<ImplContactRepository>>>
) -> ApiResponse<ResUpdateContactDto> {
    let operation = contact_usecase.update_contact(user.id, Uuid::parse_str(&contact_id).unwrap(), contact_data.into_inner()).await;
    match operation {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(e) => {
            return Err(e.into())
        }
    }
}





#[utoipa::path(
    delete,
    path = "/contact/{contact_id}",
    summary = "Delete contact",
    description = "Delete contact",
    tags = ["contact"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_id" = String,Path, description = "Customer ID")
    ),
    responses(
        (status = 200, description = "Customer deleted"),
        (status = 400, description = "Invalid contact"),
        (status = 404, description = "Customer not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[delete("/<contact_id>")]
pub async fn delete_contact(
    user: AuthenticatedUser,
    contact_id : String,
    contact_usecase: &State<Arc<ContactUseCase<ImplContactRepository>>>
) -> ApiResponse<String> {
    let operation = contact_usecase.delete_contact(user.id, Uuid::parse_str(&contact_id).unwrap()).await;
    match operation {
        Ok(_) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: "Contact deleted".to_string()
            })
        },
        Err(e) => {
            return Err(e.into())
        }
    }
}



