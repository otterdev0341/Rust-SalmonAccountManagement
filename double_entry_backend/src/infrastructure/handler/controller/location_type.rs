use rocket::{delete, get, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{auth_dto::AuthenticatedUser, location_type_dto::{ReqCreateLocationTypeDto, ReqUpdateLocationTypeDto, ResEntryLocationTypeDto, ResListLocationTypeDto}}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};

pub fn location_type_routes() -> Vec<Route> {
    routes![
        create_location_type,
        view_location_type,
        view_location_types,
        edit_location_type,
        delete_location_type,
        options
    ]
}



#[utoipa::path(
    post,
    path = "/location-type",
    request_body = ReqCreateLocationTypeDto,
    summary = "Create new location type",
    description = "Create location type",
    tags = ["location_type"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Location type created"),
        (status = 400, description = "Invalid location type name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/location_type", format = "json", data = "<location_type_data>")]
pub async fn create_location_type(
    user: AuthenticatedUser,
    location_type_data: Json<ReqCreateLocationTypeDto>
) -> ApiResponse<String> {
    todo!( )
}




#[utoipa::path(
    get,
    path = "/location-type/{location_type_id}",
    summary = "View location type",
    description = "View location type",
    tags = ["location_type"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("location_type_id" = String, Path, description = "Location type id")
    ),
    responses(
        (status = 200, description = "Location type found",
            body = ResEntryLocationTypeDto,
            description = "Location type found"
        ),
        (status = 400, description = "Invalid location type id"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/<location_type_id>", format = "json")]
pub async fn view_location_type(
    user: AuthenticatedUser,
    location_type_id: String
) -> ApiResponse<ResEntryLocationTypeDto> {
    todo!()
}



#[utoipa::path(
    get,
    path = "/location-type",
    summary = "View location types",
    description = "View location types",
    tags = ["location_type"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Location types found",
            body = ResListLocationTypeDto,
            description = "Location types found"
        ),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/", format = "json")]
pub async fn view_location_types(
    user: AuthenticatedUser
) -> ApiResponse<ResListLocationTypeDto> {
    todo!()
}





#[utoipa::path(
    put,
    path = "/location-type/{location_type_id}",
    request_body = ReqUpdateLocationTypeDto,
    summary = "Edit location type",
    description = "Edit location type",
    tags = ["location_type"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("location_type_id" = String, Path, description = "Location type id")
    ),
    responses(
        (status = 200, description = "Location type edited"),
        (status = 400, description = "Invalid location type name or description"),
        (status = 404, description = "Location type not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/<location_type_id>", format = "json", data = "<location_type_data>")]
pub async fn edit_location_type(
    user: AuthenticatedUser,
    location_type_id: String,
    location_type_data: Json<ReqUpdateLocationTypeDto>
) -> ApiResponse<String> {
    todo!()
}




#[utoipa::path(
    delete,
    path = "/location-type/{location_type_id}",
    summary = "Delete location type",
    description = "Delete location type",
    tags = ["location_type"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("location_type_id" = String, Path, description = "Location type id")
    ),
    responses(
        (status = 200, description = "Location type deleted"),
        (status = 400, description = "Invalid location type id"),
        (status = 404, description = "Location type not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/<location_type_id>", format = "json")]
pub async fn delete_location_type(
    user: AuthenticatedUser,
    location_type_id: String
) -> ApiResponse<String> {
    todo!()
}