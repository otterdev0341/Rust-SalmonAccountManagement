use std::sync::Arc;

use rocket::{delete, get, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;

use crate::{application::usecase::contact_type_usecase::{ContactTypeUseCase, ContactTypeUseCaseError}, domain::{dto::{auth_dto::AuthenticatedUser, contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto, ResEntryContactTypeDto, ResListContactTypeDto, ResUpdateContactTypeDto}, std_201::ResCreateSuccess}, entities::{contact_type, prelude::ContactType}}, infrastructure::{faring::cors::options, handler::api_response::{api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, api_success_response::{ApiCreatedResponse, ApiCreatedResponseType}}, mysql::repositories::impl_contact_type_repository::ImplContactTypeRepository}};




pub fn contact_type_routes() -> Vec<Route> {
    routes![
        create_contact_type,
        view_contact_type,
        view_contact_types,
        update_contact_type,
        delete_contact_type,
        options
    ]
}


impl From<ContactTypeUseCaseError> for ApiErrorResponse {
    fn from(error: ContactTypeUseCaseError) -> Self {
        match error {
            ContactTypeUseCaseError::ContactTypeNotFound => ApiErrorResponse::new(404, "Contact Type not found".to_string()),
            ContactTypeUseCaseError::InternalServerError => ApiErrorResponse::new(500, "Internal server error".to_string()),
            ContactTypeUseCaseError::UnAuthorized => ApiErrorResponse::new(401, "UnAuthorized".to_string()),
            ContactTypeUseCaseError::DeleteFailed => ApiErrorResponse::new(500, "Delete failed".to_string()),
            ContactTypeUseCaseError::UpdateFailed => ApiErrorResponse::new(500, "Update failed".to_string())

        }
    }
}


#[utoipa::path(
    post,
    path = "/contact-type",
    request_body = ReqCreateContactTypeDto,
    summary = "Create new contact type",
    description = "Create contact type",
    tags = ["contact-type"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Contact Type created"),
        (status = 400, description = "Invalid contact type name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/", format = "json", data = "<contact_type_data>")]
pub async fn create_contact_type(
    user: AuthenticatedUser,
    contact_type_data: Json<ReqCreateContactTypeDto>,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ImplContactTypeRepository>>>
) -> ApiCreatedResponseType<ResCreateSuccess>   {

    if contact_type_data.name.is_empty() || contact_type_data.description.is_empty() {
        return Err(ApiErrorResponse::new(400, "Invalid contact type name or description".to_string()).into());
    }

    let opration = contact_type_usecase.create_contact_type(user.id, contact_type_data.into_inner()).await;
    
    match opration {
        Ok(data) => {
            return Ok(ApiCreatedResponse{
                status: "success".to_string(),
                message: "Contact Type created".to_string(),
                data: data
            })
        },
        Err(e) => Err(e.into())
    }
    
}







#[utoipa::path(
    get,
    path = "/contact-type/{contact_type_id}",
    summary = "View contact type",
    description = "View contact type",
    tags = ["contact-type"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_type_id" = String,Path, description = "Contact Type ID")
    ),
    responses(
        (status = 200, description = "contact type found",
            body = ResEntryContactTypeDto,
            description = "contact type data"
    ),
        (status = 400, description = "Invalid contact type"),
        (status = 404, description = "contact type not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/<contact_type_id>")]
pub async fn view_contact_type(
    user: AuthenticatedUser,
    contact_type_id: String,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ImplContactTypeRepository>>>
) -> ApiResponse<ResEntryContactTypeDto>  {
    let operation = contact_type_usecase.get_contact_type(user.id, Uuid::parse_str(&contact_type_id).unwrap()).await;
    match operation {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(e) => Err(e.into())
    }
}




#[utoipa::path(
    get,
    path = "/contact-type",
    summary = "View contact types",
    description = "View contact types",
    tags = ["contact-type"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "contact types found",
            body = ResListContactTypeDto,
            description = "contact types data"
    ),
        (status = 400, description = "Invalid contact types"),
        (status = 404, description = "Customers not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/")]
pub async fn view_contact_types(
    user: AuthenticatedUser,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ImplContactTypeRepository>>>
) -> ApiResponse<ResListContactTypeDto>  {
    let operation = contact_type_usecase.get_contact_types(user.id).await;
    match operation {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(e) => Err(e.into())
    }
}




#[utoipa::path(
    put,
    path = "/contact-type/{contact_type_id}",
    request_body = ReqUpdateContactTypeDto,
    summary = "Update contact type",
    description = "Update contact type",
    tags = ["contact-type"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_type_id" = String,Path, description = "Contact Type ID")
    ),
    responses(
        (status = 200, description = "Contact Type updated"),
        (status = 400, description = "Invalid contact type name or description"),
        (status = 404, description = "Contact Type not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/<contact_type_id>", format = "json", data = "<contact_type_data>")]
pub async fn update_contact_type(
    user: AuthenticatedUser,
    contact_type_id: String,
    contact_type_data: Json<ReqUpdateContactTypeDto>,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ImplContactTypeRepository>>>
) 
-> ApiResponse<ResUpdateContactTypeDto>  {
    let operation = contact_type_usecase.update_contact_type(user.id, Uuid::parse_str(&contact_type_id).unwrap(), contact_type_data.into_inner()).await;
    match operation {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(e) => Err(e.into())
    }
}




#[utoipa::path(
    delete,
    path = "/contact-type/{contact_type_id}",
    summary = "Delete contact type",
    description = "Delete contact type",
    tags = ["contact-type"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("contact_type_id" = String,Path, description = "Contact Type ID")
    ),
    responses(
        (status = 200, description = "Contact Type deleted"),
        (status = 400, description = "Invalid contact type"),
        (status = 404, description = "Contact Type not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/<contact_type_id>")]
pub async fn delete_contact_type(
    user: AuthenticatedUser,
    contact_type_id: String,
    contact_type_usecase: &State<Arc<ContactTypeUseCase<ImplContactTypeRepository>>>
) 
-> ApiResponse<String>  {
    let operation = contact_type_usecase.delete_contact_type(user.id, Uuid::parse_str(&contact_type_id).unwrap()).await;
    match operation {
        Ok(_) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: "Contact Type deleted".to_string()
            })
        },
        Err(e) => Err(e.into())
    }
}
