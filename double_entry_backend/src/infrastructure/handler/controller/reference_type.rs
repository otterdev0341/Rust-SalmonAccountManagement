use rocket::{delete, get, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{account_system::reference_type_dto::{ReqCreateReferenceTypeDto, ReqUpdateReferenceTypeDto, ResEntryReferenceTypeDto, ResListReferenceTypeDto}, auth_dto::AuthenticatedUser}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};





pub fn reference_type_routes() -> Vec<Route> {
    
    routes![
        create_reference_type,
        edit_reference_type,
        view_reference_type,
        view_reference_types,
        delete_reference_type,
        options
    ]
}


#[utoipa::path(
    post,
    path = "/reference-type",
    summary = "Create reference type",
    description = "Create reference type",
    tags = ["reference-type"],
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateReferenceTypeDto,
    responses(
        (status = 200, description = "Reference type created"),
        (status = 400, description = "Invalid reference type"),
        (status = 500, description = "Internal server error")
    ),
)]
#[post("/", format = "json", data = "<reference_type_data>")]
pub async fn create_reference_type(
    user: AuthenticatedUser,
    reference_type_data: Json<ReqCreateReferenceTypeDto>,
) -> ApiResponse<String> {
    todo!()
}




#[utoipa::path(
    put,
    path = "/reference-type",
    summary = "Edit reference type",
    description = "Edit reference type",
    tags = ["reference-type"],
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqUpdateReferenceTypeDto,
    responses(
        (status = 200, description = "Reference type updated"),
        (status = 400, description = "Invalid reference type"),
        (status = 500, description = "Internal server error")
    ),
)]
#[put("/<reference_id>", format = "json", data = "<reference_type_data>")]
pub async fn edit_reference_type(
    user: AuthenticatedUser,
    reference_id: String,
    reference_type_data: Json<ReqUpdateReferenceTypeDto>
) 
-> ApiResponse<String> {
    todo!()
}




#[utoipa::path(
    get,
    path = "/reference-type/{reference_id}",
    summary = "View reference type",
    description = "View reference type",
    tags = ["reference-type"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Reference type found",
            body = ResEntryReferenceTypeDto,
            description = "Reference type found"
    ),
        (status = 404, description = "Reference type not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/<reference_id>", format = "json")]
pub async fn view_reference_type(
    user: AuthenticatedUser,
    reference_id: String
) 
-> ApiResponse<ResEntryReferenceTypeDto> {
    todo!()
}



#[utoipa::path(
    get,
    path = "/reference-type",
    summary = "View reference types",
    description = "View reference types",
    tags = ["reference-type"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Reference types found",
            body = ResListReferenceTypeDto,
            description = "Reference types found"
        ),
        (status = 404, description = "Reference types not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/", format = "json")]
pub async fn view_reference_types(
    user: AuthenticatedUser
) -> ApiResponse<ResListReferenceTypeDto> {
    todo!()
}




#[utoipa::path(
    delete,
    path = "/reference-type/{reference_id}",
    summary = "Delete reference type",
    description = "Delete reference type",
    tags = ["reference-type"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Reference type deleted"),
        (status = 404, description = "Reference type not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[delete("/<reference_id>", format = "json")]
pub async fn delete_reference_type(
    user: AuthenticatedUser,
    reference_id: String
) -> ApiResponse<String> {
    todo!()
}