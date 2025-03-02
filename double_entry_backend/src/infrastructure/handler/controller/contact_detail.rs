// usecase create_customer_contact
// usecase view_customer_contact
// usecase view_customer_contacts


// usecase edit_customer_contact
// usecase delete_customer_contact

use std::sync::Arc;

use rocket::{delete, get, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;

use crate::{application::usecase::contact_detail_usecase::{ContactDetailUseCase, ContactDetailUseCaseError}, domain::{dto::{auth_dto::AuthenticatedUser, contact_detail_dto::{ReqCreateContactDetailDto, ReqUpdateContactDetailDto, ResEntryContactDetailDto, ResListContactDetailDto, ResUpdateContactDetailDto}, std_201::ResCreateSuccess}, entities::prelude::ContactDetail}, infrastructure::{faring::cors::options, handler::api_response::{api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, api_success_response::{ApiCreatedResponse, ApiCreatedResponseType}}, mysql::repositories::impl_contact_detail::ImplContactDetailRespository}};

use super::contact;

pub fn contact_detail_routes() -> Vec<Route> {
    routes![
        create_contact_detail,
        view_contact_detail,
        view_contact_details,
        edit_contact_detail,
        delete_contact_detail,
        options
    ]
}

impl From<ContactDetailUseCaseError> for ApiErrorResponse {
    fn from(error: ContactDetailUseCaseError) -> Self {
        match error {
            ContactDetailUseCaseError::ContactDetailNotFound => ApiErrorResponse::new(404, "Contact Detail not found".to_string()),
            ContactDetailUseCaseError::InternalServerError => ApiErrorResponse::new(500, "Internal server error".to_string()),
            ContactDetailUseCaseError::DeleteFailed => ApiErrorResponse::new(500, "Delete failed".to_string()),
            ContactDetailUseCaseError::UpdateFailed => ApiErrorResponse::new(500, "Update failed".to_string())

        }
    }
}


#[utoipa::path(
    post,
    path = "/contact-detail",
    request_body = ReqCreateContactDetailDto,
    summary = "Create new contact detail",
    description = "Create contact detail",
    tags = ["contact-detail"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Contact Detail created"),
        (status = 400, description = "Invalid contact detail name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/<contact_id>", format = "json", data = "<contact_detail_data>")]
pub async fn create_contact_detail(
    user : AuthenticatedUser,
    contact_detail_data: Json<ReqCreateContactDetailDto>,
    contact_id: String,
    contact_detail_usecase: &State<Arc<ContactDetailUseCase<ImplContactDetailRespository>>>
)
-> ApiCreatedResponseType<ResCreateSuccess> {

    if contact_id.is_empty() {
        return Err(ApiErrorResponse::new(400, "Invalid contact id".to_string()));
    }

    let operation = 
        contact_detail_usecase.create_contact_detail(
            user.id, 
            Uuid::parse_str(&contact_id).unwrap(),
            contact_detail_data.into_inner(),
        ).await;
    match operation {
        Ok(data) => {
            Ok(ApiCreatedResponse{
                status: "success".to_string(),
                message: "Contact Detail created".to_string(),
                data: data
            })
        },
        Err(e) => Err(e.into())
    }
}



#[utoipa::path(
    get,
    path = "/contact-detail/{contact_detail_id}",
    summary = "View contact detail",
    description = "View contact detail",
    tags = ["contact-detail"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "contact detail found",
            body = ResEntryContactDetailDto,
            description = "contact detail data"
    ),
        (status = 400, description = "Invalid contact detail"),
        (status = 404, description = "contact detail not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/<contact_detail_id>")]
pub async fn view_contact_detail(
    user : AuthenticatedUser,
    contact_detail_id : String,
    contact_detail_usecase: &State<Arc<ContactDetailUseCase<ImplContactDetailRespository>>>
) -> ApiResponse<ResEntryContactDetailDto> {
    let operaion = contact_detail_usecase.get_contact_detail(user.id, Uuid::parse_str(&contact_detail_id).unwrap()).await;
    match operaion {
        Ok(data) => {
            Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(e) => Err(e.into())
    }
}



#[utoipa::path(
    get,
    path = "/contact-detail",
    summary = "View customer contacts",
    description = "View customer contacts",
    tags = ["contact-detail"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Contact details found",
            body = ResListContactDetailDto,
            description = "Contact details data"
    ),
        (status = 400, description = "Invalid customer contacts"),
        (status = 404, description = "Contact details not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/")]
pub async fn view_contact_details(
    user : AuthenticatedUser,
    contact_detail_usecase: &State<Arc<ContactDetailUseCase<ImplContactDetailRespository>>>
) -> ApiResponse<ResListContactDetailDto> {
    let operation = contact_detail_usecase.get_contact_details(user.id).await;
    match operation {
        Ok(data) => {
            Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(e) => Err(e.into())
    }
}



#[utoipa::path(
    put,
    path = "/contact-detail/{contact_detail_id}",
    request_body = ReqUpdateContactDetailDto,
    summary = "Edit contact detail",
    description = "Edit contact detail",
    tags = ["contact-detail"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "contact detail edited"),
        (status = 400, description = "Invalid contact detail name or description"),
        (status = 404, description = "contact detail not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/<contact_detail_id>", format = "json", data = "<customer_contact_data>")]
pub async fn edit_contact_detail(
    user : AuthenticatedUser,
    contact_detail_id : String,
    customer_contact_data: Json<ReqUpdateContactDetailDto>,
    contact_detail_usecase: &State<Arc<ContactDetailUseCase<ImplContactDetailRespository>>>
) -> ApiResponse<ResUpdateContactDetailDto> {
    let operation = contact_detail_usecase.update_contact_detail(
        user.id,
        Uuid::parse_str(&contact_detail_id).unwrap(),
        customer_contact_data.into_inner()
    ).await;
    match operation {
        Ok(data) => {
            Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(e) => Err(e.into())
    }
}



#[utoipa::path(
    delete,
    path = "/contact-detail/{contact_detail_id}",
    summary = "Delete contact detail",
    description = "Delete contact detail",
    tags = ["contact-detail"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "contact detail deleted"),
        (status = 404, description = "contact detail not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/<contact_detail_id>")]
pub async fn delete_contact_detail(
    user : AuthenticatedUser,
    contact_detail_id : String,
    contact_detail_usecase: &State<Arc<ContactDetailUseCase<ImplContactDetailRespository>>>
) -> ApiResponse<String> {
    let operation = contact_detail_usecase.delete_contact_detail(user.id, Uuid::parse_str(&contact_detail_id).unwrap()).await;
    match operation {
        Ok(_) => {
            Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: "Contact detail deleted".to_string()
            })
        },
        Err(e) => Err(e.into())
    }
}