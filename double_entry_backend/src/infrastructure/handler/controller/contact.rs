use rocket::{delete, get, post, put, routes, serde::json::Json, Route};
use crate::{domain::dto::{auth_dto::AuthenticatedUser, contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto}}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};


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
    contact_id : String
) -> ApiResponse<ResEntryContactDto> {
    todo!()
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
    user: AuthenticatedUser
) -> ApiResponse<ResListContactDto> {
    todo!()
    
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
    contact_id : Json<ReqCreateContactDto>
) -> ApiResponse<String> {
    todo!()
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
    contact_data : Json<ReqUpdateContactDto>
) -> ApiResponse<String> {
    todo!()
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
    contact_id : String
) -> ApiResponse<String> {
    todo!()
}



