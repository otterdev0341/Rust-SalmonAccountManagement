// usecase create_customer_contact
// usecase view_customer_contact
// usecase view_customer_contacts


// usecase edit_customer_contact
// usecase delete_customer_contact

use rocket::{delete, get, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{auth_dto::AuthenticatedUser, customer_contact_dto::{ReqCreateCustomerContactDto, ReqUpdateCustomerContactDto, ResEntryCustomerContactDto, ResListCustomerContactDto}, customer_dto::ResEntryCustomerDto}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};

pub fn customer_contact_routes() -> Vec<Route> {
    routes![
        create_customer_contact,
        view_customer_contact,
        view_customer_contacts,
        edit_customer_contact,
        delete_customer_contact,
        options
    ]
}



#[utoipa::path(
    post,
    path = "/customer-contact",
    request_body = ReqCreateCustomerContactDto,
    summary = "Create new customer contact",
    description = "Create customer contact",
    tags = ["customer-contact"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Customer contact created"),
        (status = 400, description = "Invalid customer contact name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/", format = "json", data = "<customer_contact_data>")]
pub async fn create_customer_contact(
    user : AuthenticatedUser,
    customer_contact_data: Json<ReqCreateCustomerContactDto>
)
-> ApiResponse<String> {
    todo!()
}



#[utoipa::path(
    get,
    path = "/customer-contact/{customer_contact_id}",
    summary = "View customer contact",
    description = "View customer contact",
    tags = ["customer-contact"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Customer contact found",
            body = ResEntryCustomerContactDto,
            description = "Customer contact data"
    ),
        (status = 400, description = "Invalid customer contact"),
        (status = 404, description = "Customer contact not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/<customer_contact_id>")]
pub async fn view_customer_contact(
    user : AuthenticatedUser,
    customer_contact_id : String
) -> ApiResponse<ResEntryCustomerContactDto> {
    todo!()
}



#[utoipa::path(
    get,
    path = "/customer-contact",
    summary = "View customer contacts",
    description = "View customer contacts",
    tags = ["customer-contact"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Customer contacts found",
            body = ResListCustomerContactDto,
            description = "Customer contacts data"
    ),
        (status = 400, description = "Invalid customer contacts"),
        (status = 404, description = "Customer contacts not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/")]
pub async fn view_customer_contacts(
    user : AuthenticatedUser
) -> ApiResponse<ResListCustomerContactDto> {
    todo!()
}



#[utoipa::path(
    put,
    path = "/customer-contact/{customer_contact_id}",
    request_body = ReqUpdateCustomerContactDto,
    summary = "Edit customer contact",
    description = "Edit customer contact",
    tags = ["customer-contact"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Customer contact edited"),
        (status = 400, description = "Invalid customer contact name or description"),
        (status = 404, description = "Customer contact not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/<customer_contact_id>", format = "json", data = "<customer_contact_data>")]
pub async fn edit_customer_contact(
    user : AuthenticatedUser,
    customer_contact_id : String,
    customer_contact_data: Json<ReqUpdateCustomerContactDto>
) -> ApiResponse<String> {
    todo!()
}



#[utoipa::path(
    delete,
    path = "/customer-contact/{customer_contact_id}",
    summary = "Delete customer contact",
    description = "Delete customer contact",
    tags = ["customer-contact"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Customer contact deleted"),
        (status = 404, description = "Customer contact not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/<customer_contact_id>")]
pub async fn delete_customer_contact(
    user : AuthenticatedUser,
    customer_contact_id : String
) -> ApiResponse<String> {
    todo!()
}