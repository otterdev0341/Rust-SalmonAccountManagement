use rocket::{delete, get, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{auth_dto::AuthenticatedUser, contact_type_dto::{ReqCreateContactTypeDto, ReqUpdateContactTypeDto, ResEntryContactTypeDto, ResListContactTypeDto}}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};




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
    contact_type_data: Json<ReqCreateContactTypeDto>
) -> ApiResponse<String>  {
    todo!()
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
    contact_type_id: String
) -> ApiResponse<ResEntryContactTypeDto>  {
    todo!()
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
) -> ApiResponse<ResListContactTypeDto>  {
    todo!()
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
) 
-> ApiResponse<String>  {
    todo!()
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
    contact_type_id: String
) 
-> ApiResponse<String>  {
    todo!()
}
