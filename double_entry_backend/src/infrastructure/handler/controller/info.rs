use rocket::{delete, get, post, put, routes, serde::json::Json, Route};
use serde::de;
use utoipa::openapi::info;
use crate::{domain::dto::{auth_dto::AuthenticatedUser, info_dto::{ReqCreateInfoDto, ReqUpdateInfoDto, ResEntryInfoDto, RestListInfoDto}}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};


pub fn info_routes() -> Vec<Route> {
    routes![
        create_project_info,
        view_project_info,
        view_project_infos,
        edit_project_info,
        delete_project_info,
        options
    ]
}



#[utoipa::path(
    post,
    path = "/info",
    request_body = ReqCreateInfoDto,
    summary = "Create new project info",
    description = "Create project info",
    tags = ["info"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Project info created"),
        (status = 400, description = "Invalid project info name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/", format = "json", data = "<info_data>")]
pub async fn create_project_info(
    user: AuthenticatedUser,
    info_data: Json<ReqCreateInfoDto>
    ) 
-> ApiResponse<String> {
    todo!()
}








#[utoipa::path(
    get,
    path = "/{info_id}",
    summary = "View project infos",
    description = "View project infos",
    tags = ["info"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("info_id" = String,Path, description = "Project info id")
    ),
    responses(
        (status = 200, description = "Project infos found",
            body = ResEntryInfoDto,
            description = "Project infos found"
    ),
        (status = 404, description = "Project infos not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/<info_id>", format = "json")]
pub async fn view_project_info(
    user: AuthenticatedUser,
    info_id: String
) -> ApiResponse<ResEntryInfoDto> {
    todo!()
}





#[utoipa::path(
    get,
    path = "/info",
    summary = "View project infos",
    description = "View project infos",
    tags = ["info"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Project infos found",
            body = RestListInfoDto,
            description = "Project infos found"
        ),
        (status = 404, description = "Project infos not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/info", format = "json")]
pub async fn view_project_infos(
    user: AuthenticatedUser
) -> ApiResponse<RestListInfoDto> {
    todo!()
}



#[utoipa::path(
    put,
    path = "/{info_id}",
    request_body = ReqUpdateInfoDto,
    summary = "Edit project info",
    description = "Edit project info",
    tags = ["info"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("info_id" = String,Path, description = "Project info id to update")
    ),
    responses(
        (status = 200, description = "Project info edited"),
        (status = 400, description = "Invalid project info name or description"),
        (status = 404, description = "Project info not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/info/<info_id>", format = "json", data = "<info_data>")]
pub async fn edit_project_info(
    user: AuthenticatedUser,
    info_id: String,
    info_data: Json<ReqUpdateInfoDto>
    ) 
-> ApiResponse<String> {
    todo!()
}



#[utoipa::path(
    delete,
    path = "/{info_id}",
    summary = "Delete project info",
    description = "Delete project info",
    tags = ["info"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("info_id" = String,Path, description = "Project info id to delete")
    ),
    responses(
        (status = 200, description = "Project info deleted"),
        (status = 404, description = "Project info not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/info/<info_id>", format = "json")]
pub async fn delete_project_info(
    user: AuthenticatedUser,
    info_id: String
) 
-> ApiResponse<String> {
    todo!()
}

