use std::str::FromStr;

use rocket::{delete, get, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;

use crate::{application::usecase::project_usecase::{ProjectUseCase, ProjectUseCaseError}, domain::dto::{auth_dto::AuthenticatedUser, project_dto::{ReqCreateProjectDto, ReqUpdateProjectDto, ResEntryProjectDto, ResListProjectDto, ResUpdateProjectDto}, std_201::ResCreateSuccess}, infrastructure::{faring::cors::options, handler::api_response::{api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, api_success_response::{ApiCreatedResponse, ApiCreatedResponseType}, api_update_response::{ApiUpdateResponse, ApiUpdateResponseType}}, mysql::repositories::impl_project_repository::ImplProjectRepository}};

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


impl From<ProjectUseCaseError> for ApiErrorResponse {
    fn from(error: ProjectUseCaseError) -> Self {
        match error {
            ProjectUseCaseError::ProjectNotFound => ApiErrorResponse::new(404, "Project not found".to_string()),
            ProjectUseCaseError::InternalServerError => ApiErrorResponse::new(500, "Internal server error".to_string()),
            ProjectUseCaseError::ConflictingCompany => ApiErrorResponse::new(409, "Conflicting company".to_string()),
            ProjectUseCaseError::ConvertUuidError => ApiErrorResponse::new(500, "Uuid convert error".to_string()),
            ProjectUseCaseError::DeleteFailed => ApiErrorResponse::new(500, "Delete failed".to_string()),
            ProjectUseCaseError::UpdateFailed => ApiErrorResponse::new(500, "Update failed".to_string())
        }
    }
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
    project_data: Json<ReqCreateProjectDto>,
    project_usecase: &State<ProjectUseCase<ImplProjectRepository>>
) -> ApiCreatedResponseType<ResCreateSuccess> {
    let operation = 
            project_usecase
                .create_project(user.id, project_data.company_id, project_data.into_inner()).await;

    match operation {
        Ok(data) => {
            return Ok(ApiCreatedResponse{
                status: "success".to_string(),
                message: "Project created".to_string(),
                data: data
            })
        },
        Err(err) => {
            return Err(err.into());
        }
    }
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
    project_id: String,
    project_usecase: &State<ProjectUseCase<ImplProjectRepository>>
) -> ApiResponse<ResEntryProjectDto> {
    let operation = 
            project_usecase
                .get_project(user.id, Uuid::from_str(&project_id).unwrap()).await;
    match operation {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(err) => {
            return Err(err.into());
        }
    }
    
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
    user: AuthenticatedUser,
    project_usecase: &State<ProjectUseCase<ImplProjectRepository>>
) -> ApiResponse<ResListProjectDto> {
    let operation = 
            project_usecase
                .get_projects(user.id).await;
    match operation {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: data
            })
        },
        Err(err) => {
            return Err(err.into());
        }
    }
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
    project_data: Json<ReqUpdateProjectDto>,
    project_usecase: &State<ProjectUseCase<ImplProjectRepository>>
    ) 
-> ApiUpdateResponseType<ResUpdateProjectDto> {
    
    let operation = 
            project_usecase
                .update_project(user.id, Uuid::from_str(&project_id).unwrap(), project_data.into_inner()).await;
    
    match operation {
        Ok(data) => {
            return Ok(ApiUpdateResponse{
                status: "success".to_string(),
                message: "Project edited".to_string(),
                data: data
            })
        },
        Err(err) => {
            return Err(err.into());
        }
    }
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
    project_id: String,
    project_usecase: &State<ProjectUseCase<ImplProjectRepository>>
) -> ApiResponse<String> {
    let operation = 
            project_usecase
                .delete_project(user.id, Uuid::from_str(&project_id).unwrap()).await;
    match operation {
        Ok(_) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: "Project deleted".to_string()
            })
        },
        Err(err) => {
            return Err(err.into());
        }
    }
}




