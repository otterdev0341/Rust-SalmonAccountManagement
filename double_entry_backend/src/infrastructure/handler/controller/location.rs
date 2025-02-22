use rocket::{delete, get, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{auth_dto::AuthenticatedUser, location_dto::{ReqCreateLocationDto, ReqUpdateLocationDto, ResEntryLocationDto, ResListLocationDto}}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};



pub fn location_routes() -> Vec<Route> {
    routes![
        create_location,
        view_location,
        view_locations,
        edit_location,
        delete_location,
        options
    ]
}



#[utoipa::path(
    post,
    path = "/location",
    request_body = ReqCreateLocationDto,
    summary = "Create new location",
    description = "Create location",
    tags = ["location"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Location created"),
        (status = 400, description = "Invalid location name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/", format = "json", data = "<location_data>")]
pub async fn create_location(
    user: AuthenticatedUser,
    location_data: Json<ReqCreateLocationDto>
) -> ApiResponse<String> {
    todo!()
}




#[utoipa::path(
    get,
    path = "location/{location_id}",
    summary = "View location",
    description = "View location",
    tags = ["location"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("location_id" = String, Path, description = "Location id")
    ),
    responses(
        (status = 200, description = "Location found",
            body = ResEntryLocationDto, 
            description = "Location found"
        ),
        (status = 400, description = "Invalid location id"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/<location_id>", format = "json")]
pub async fn view_location(
    user: AuthenticatedUser,
    location_id: String
) -> ApiResponse<ResEntryLocationDto> {
    todo!()
}



#[utoipa::path(
    get,
    path = "/locations",
    summary = "View locations",
    description = "View locations",
    tags = ["location"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Locations found",
            body = ResListLocationDto,
            description = "Locations found"
        ),
        (status = 404, description = "Locations not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/", format = "json")]
pub async fn view_locations(
    user: AuthenticatedUser
) -> ApiResponse<ResListLocationDto> {
    todo!()
}



#[utoipa::path(
    put,
    path = "/location/{location_id}",
    request_body = ReqUpdateLocationDto,
    summary = "Edit location",
    description = "Edit location",
    tags = ["location"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("location_id" = String, Path, description = "Location id")
    ),
    responses(
        (status = 200, description = "Location updated"),
        (status = 400, description = "Invalid location id"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/<location_id>", format = "json", data = "<location_data>")]
pub async fn edit_location(
    user: AuthenticatedUser,
    location_id: String,
    location_data: Json<ReqUpdateLocationDto>
) -> ApiResponse<String> {
    todo!()
}




#[utoipa::path(
    delete,
    path = "/location/{location_id}",
    summary = "Delete location",
    description = "Delete location",
    tags = ["location"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("location_id" = String, Path, description = "Location id")
    ),
    responses(
        (status = 200, description = "Location deleted"),
        (status = 400, description = "Invalid location id"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/<location_id>", format = "json")]
pub async fn delete_location(
    user: AuthenticatedUser,
    location_id: String
) 
-> ApiResponse<String> {
    todo!()
}
