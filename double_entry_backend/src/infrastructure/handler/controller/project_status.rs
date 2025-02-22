use std::default;

use rocket::{delete, get, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{auth_dto::AuthenticatedUser, project_status::{ReqCreateProjectStatusDto, ReqUpdateProjectStatusDto, ResEntryProjectStatusDto, ResListProjectStatusDto}}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};

pub fn project_status_routes() -> Vec<Route> {
    routes![
        
        options
    ]
}





#[utoipa::path(
    post,
    path = "/project-status",
    request_body = ReqCreateProjectStatusDto,
    summary = "Create new project status",
    description = "Create project status",
    tags = ["project_status"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Project status created"),
        (status = 400, description = "Invalid project status name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/", format = "json", data = "<project_status_data>")]
pub async fn create_project_status(
    user: AuthenticatedUser,
    project_status_data: Json<ReqCreateProjectStatusDto>

) -> ApiResponse<String> {
    todo!()
}




#[utoipa::path(
    get,
    path = "/project-status/{project_status_id}",
    summary = "View project status",
    description = "View project status",
    tags = ["project_status"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("project_status_id" = String, Path, description = "Project status id")
    ),
    responses(
        (status = 200, description = "Project status found",
            body = ResEntryProjectStatusDto,
            content_type = "application/json"),
        (status = 404, description = "Project status not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/<project_status_id>", format = "json")]
pub async fn view_project_status(
    user: AuthenticatedUser,
    project_status_id: String
) 
-> ApiResponse<ResEntryProjectStatusDto> {
    todo!()
}


#[utoipa::path(
    get,
    path = "/project-status",
    summary = "View project statuses",
    description = "View project statuses",
    tags = ["project_status"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Project statuses found",
            body = ResListProjectStatusDto,
            content_type = "application/json"),
        (status = 404, description = "Project statuses not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/", format = "json")]
pub async fn view_project_statuses(
    user: AuthenticatedUser
) 
-> ApiResponse<ResListProjectStatusDto> {
    todo!()
}




#[utoipa::path(
    put,
    path = "/project-status/{project_status_id}",
    request_body = ReqUpdateProjectStatusDto,
    summary = "Edit project status",
    description = "Edit project status",
    tags = ["project_status"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("project_status_id" = String, Path, description = "Project status id")
    ),
    responses(
        (status = 200, description = "Project status updated"),
        (status = 400, description = "Invalid project status name or description"),
        (status = 404, description = "Project status not found"),
        (status = 409, description = "Project status has dependencies"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/<project_status_id>", format = "json", data = "<project_status_data>")]
pub async fn edit_project_status(
    user: AuthenticatedUser,
    project_status_id: String,
    project_status_data: Json<ReqUpdateProjectStatusDto>
) 
-> ApiResponse<String> {
    todo!()
}






#[utoipa::path(
    delete,
    path = "/project-status/{project_status_id}",
    summary = "Delete project status",
    description = "Delete project status",
    tags = ["project_status"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("project_status_id" = String, Path, description = "Project status id")
    ),
    responses(
        (status = 200, description = "Project status deleted"),
        (status = 404, description = "Project status not found"),
        (status = 409, description = "Project status has dependencies"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/<project_status_id>", format = "json")]
pub async fn delete_project_status(
    user: AuthenticatedUser,
    project_status_id: String
)
-> ApiResponse<String> {
    todo!()
}

