use rocket::{delete, get, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{auth_dto::AuthenticatedUser, project_dto::{ReqCreateProjectDto, ReqUpdateProjectDto, ResEntryProjectDto, ResListProjectDto}}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};

pub fn project_routes() -> Vec<Route>{
    routes![
        create_project,
        view_project,
        view_projects,
        edit_project,
        delete_project,
        options
    ]
}




#[utoipa::path(
    post,
    path = "/project",
    request_body = ReqCreateProjectDto,
    summary = "Create new project",
    description = "Create project",
    tags = ["project"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Project created"),
        (status = 400, description = "Invalid project name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/", format = "json", data = "<project_data>")]
pub async fn create_project(
    user: AuthenticatedUser,
    project_data: Json<ReqCreateProjectDto>
) -> ApiResponse<String> {
    todo!()
}




#[utoipa::path(
    get,
    path = "project/{project_id}",
    summary = "View project",
    description = "View project",
    tags = ["project"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("project_id" = String, Path, description = "Project id")
    ),
    responses(
        (status = 200, description = "Project found",
            body = ResEntryProjectDto,
            description = "Project found"
    ),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/<project_id>", format = "json")]
pub async fn view_project(
    user: AuthenticatedUser,
    project_id: String
) -> ApiResponse<ResEntryProjectDto> {
    todo!()
}





#[utoipa::path(
    get,
    path = "/projects",
    summary = "View projects",
    description = "View projects",
    tags = ["project"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Projects found",
            body = ResListProjectDto,
            description = "Projects found"
    ),
        (status = 404, description = "Projects not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/", format = "json")]
pub async fn view_projects(
    user: AuthenticatedUser
) -> ApiResponse<ResListProjectDto> {
    todo!()
}





#[utoipa::path(
    put,
    path = "/project/{project_id}",
    request_body = ReqUpdateProjectDto,
    summary = "Edit project",
    description = "Edit project",
    tags = ["project"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("project_id" = String, Path, description = "Project id")
    ),
    responses(
        (status = 200, description = "Project edited"),
        (status = 400, description = "Invalid project name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/<project_id>", format = "json", data = "<project_data>")]
pub async fn edit_project(
    user: AuthenticatedUser,
    project_id: String,
    project_data: Json<ReqUpdateProjectDto>
    ) 
-> ApiResponse<String> {
    todo!()
}



#[utoipa::path(
    delete,
    path = "/project/{project_id}",
    summary = "Delete project",
    description = "Delete project",
    tags = ["project"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("project_id" = String, Path, description = "Project id")
    ),
    responses(
        (status = 200, description = "Project deleted"),
        (status = 404, description = "Project not found"),
        (status = 409, description = "Project has dependencies"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/<project_id>", format = "json")]
pub async fn delete_project(
    user: AuthenticatedUser,
    project_id: String
) -> ApiResponse<String> {
    todo!()
}