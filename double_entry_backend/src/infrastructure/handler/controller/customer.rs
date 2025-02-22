use rocket::{delete, get, post, put, routes, serde::json::Json, Route};
use crate::{domain::dto::{auth_dto::AuthenticatedUser, company_dto::ReqUpdateCompanyDto, customer_dto::{ReqCreateCustomerDto, ReqUpdateCustomerDto, ResEntryCustomerDto, ResListEntryCustomerDto}}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};


pub fn customer_routes() -> Vec<Route> {
    routes![
        view_customer,
        view_customers,
        create_customer,
        edit_customer,
        delete_customer,
        options
    ]
}


#[utoipa::path(
    get,
    path = "/customer/<customer_id>",
    summary = "View customer",
    description = "View customer",
    tags = ["customer"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Customer found",
            body = ResEntryCustomerDto,
            description = "Customer data"
    ),
        (status = 400, description = "Invalid customer"),
        (status = 404, description = "Customer not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/customer/<customer_id>")]
pub async fn view_customer(
    user: AuthenticatedUser,
    customer_id : String
) -> ApiResponse<ResEntryCustomerDto> {
    todo!()
}




#[utoipa::path(
    get,
    path = "/customer",
    summary = "View customers",
    description = "View customers",
    tags = ["customer"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Customers found",
            body = ResListEntryCustomerDto,
            description = "Customers data"
    ),
        (status = 400, description = "Invalid customers"),
        (status = 404, description = "Customers not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/customer")]
pub async fn view_customers(
    user: AuthenticatedUser
) -> ApiResponse<ResListEntryCustomerDto> {
    todo!()
    
}





#[utoipa::path(
    post,
    path = "/customer",
    summary = "Create customer",
    request_body = ReqCreateCustomerDto,
    description = "Create customer",
    tags = ["customer"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Customer created"),
        (status = 400, description = "Invalid customer"),
        (status = 404, description = "Customer not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[post("/customer", format = "json", data = "<customer_data>")]
pub async fn create_customer(
    user: AuthenticatedUser,
    customer_data : Json<ReqCreateCustomerDto>
) -> ApiResponse<String> {
    todo!()
}





#[utoipa::path(
    put,
    path = "/customer/{customer_id}",
    summary = "Edit customer",
    request_body = ReqUpdateCustomerDto,
    description = "Edit customer",
    tags = ["customer"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("customer_id" = String,Path, description = "Customer ID")
    ),
    responses(
        (status = 200, description = "Customer edited"),
        (status = 400, description = "Invalid customer"),
        (status = 404, description = "Customer not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[put("/customer/<customer_id>", format = "json", data = "<customer_data>")]
pub async fn edit_customer(
    user: AuthenticatedUser,
    customer_id : String,
    customer_data : Json<ReqUpdateCustomerDto>
) -> ApiResponse<String> {
    todo!()
}





#[utoipa::path(
    delete,
    path = "/customer/{customer_id}",
    summary = "Delete customer",
    description = "Delete customer",
    tags = ["customer"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("customer_id" = String,Path, description = "Customer ID")
    ),
    responses(
        (status = 200, description = "Customer deleted"),
        (status = 400, description = "Invalid customer"),
        (status = 404, description = "Customer not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[delete("/customer/<customer_id>")]
pub async fn delete_customer(
    user: AuthenticatedUser,
    customer_id : String
) -> ApiResponse<String> {
    todo!()
}



