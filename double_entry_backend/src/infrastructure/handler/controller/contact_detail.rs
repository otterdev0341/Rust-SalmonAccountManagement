// usecase create_customer_contact
// usecase view_customer_contact
// usecase view_customer_contacts


// usecase edit_customer_contact
// usecase delete_customer_contact

use rocket::{delete, get, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{auth_dto::AuthenticatedUser, contact_detail_dto::{ReqCreateContactDetailDto, ReqUpdateContactDetailDto, ResEntryContactDetailDto, ResListContactDetailDto}}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};

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
#[post("/", format = "json", data = "<contact_detail_data>")]
pub async fn create_contact_detail(
    user : AuthenticatedUser,
    contact_detail_data: Json<ReqCreateContactDetailDto>
)
-> ApiResponse<String> {
    todo!()
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
    contact_detail_id : String
) -> ApiResponse<ResEntryContactDetailDto> {
    todo!()
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
    user : AuthenticatedUser
) -> ApiResponse<ResListContactDetailDto> {
    todo!()
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
    customer_contact_data: Json<ReqUpdateContactDetailDto>
) -> ApiResponse<String> {
    todo!()
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
    contact_detail_id : String
) -> ApiResponse<String> {
    todo!()
}