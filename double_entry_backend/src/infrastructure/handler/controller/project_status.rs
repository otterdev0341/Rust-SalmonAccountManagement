use std::{default, str::FromStr, sync::Arc};

use rocket::{delete, get, post, put, routes, serde::json::Json, Route, State};
use uuid::Uuid;

use crate::{application::usecase::project_status_usecase::{ProjectStatusUseCase, ProjectStatusUseCaseError}, domain::{dto::{auth_dto::AuthenticatedUser, project_status_dto::{ReqCreateProjectStatusDto, ReqUpdateProjectStatusDto, ResEntryProjectStatusDto, ResListProjectStatusDto, ResUpdateProjectStatusDto}, std_201::ResCreateSuccess}, entities::project_status}, infrastructure::{faring::cors::options, handler::api_response::{api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, api_success_response::{ApiCreatedResponse, ApiCreatedResponseType}}, mysql::repositories::impl_project_status_repository::ImplProjectStatusRepository}};

pub fn project_status_routes() -> Vec<Route> {
    routes![
        create_project_status,
        view_project_status,
        view_project_statuses,
        edit_project_status,
        delete_project_status,
        options
    ]
}

impl From<ProjectStatusUseCaseError> for ApiErrorResponse{
    fn from(error: ProjectStatusUseCaseError) -> Self {
        match error {
            ProjectStatusUseCaseError::ProjectStatusNotFound => ApiErrorResponse::new(404, "Project status not found".to_string()),
            ProjectStatusUseCaseError::InternalServerError => ApiErrorResponse::new(500, "Internal server error".to_string()),
            ProjectStatusUseCaseError::DeleteFailed => ApiErrorResponse::new(500, "Delete failed".to_string()),
            ProjectStatusUseCaseError::CreateFailed => ApiErrorResponse::new(500, "Create failed".to_string()),
            ProjectStatusUseCaseError::UpdateFailed => ApiErrorResponse::new(500, "Update failed".to_string()),
            ProjectStatusUseCaseError::ThisProjectNameAlreadyExist => ApiErrorResponse::new(409, "This project name already exist".to_string())
        }
    }
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
    project_status_data: Json<ReqCreateProjectStatusDto>,
    project_status_usecase: &State<Arc<ProjectStatusUseCase<ImplProjectStatusRepository>>>

) -> ApiCreatedResponseType<ResCreateSuccess> {

    // validate data before operation
    if project_status_data.name.is_empty() || project_status_data.description.is_empty() {
        return Err(ApiErrorResponse::new(400, "Project status name and detail is required".to_string()))
    }

    // peform operaiont
    let operation = project_status_usecase.create_project_status(user.id, project_status_data.into_inner()).await;
    match operation {
        Ok(this_data) => {
            return Ok(ApiCreatedResponse{
                status: "success".to_string(),
                message: "project detail successfully created".to_string(),
                data: this_data

            })
        },
        Err(e) => {
            return Err(e.into())
        }
    }
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
    project_status_id: String,
    project_status_usecase: &State<Arc<ProjectStatusUseCase<ImplProjectStatusRepository>>>
) 
-> ApiResponse<ResEntryProjectStatusDto> {
    let operation 
        = project_status_usecase
            .get_project_status(user.id, Uuid::from_str(&project_status_id).unwrap()).await;
    match operation {
        Ok(this_data) => {
            return Ok(
                ApiSuccessResponse{
                    status: "success".to_string(),
                    data: this_data
                }
            )
        },
        Err(e) => {
            return Err(e.into())
        }
    }
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
    user: AuthenticatedUser,
    project_status_usecase: &State<Arc<ProjectStatusUseCase<ImplProjectStatusRepository>>>
) 
-> ApiResponse<ResListProjectStatusDto> {
    let operation = project_status_usecase.get_project_statuses(user.id).await;
    match operation {
        Ok(this_data) => {
            return Ok(
                ApiSuccessResponse{
                    status: "success".to_string(),
                    data: this_data
                }
            )
        },
        Err(e) => {
            return Err(e.into())
        }
    }
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
    project_status_data: Json<ReqUpdateProjectStatusDto>,
    project_status_usecase: &State<Arc<ProjectStatusUseCase<ImplProjectStatusRepository>>>
) 
-> ApiResponse<ResUpdateProjectStatusDto> {
    let operation 
        = project_status_usecase
            .update_project_status(user.id, Uuid::from_str(&project_status_id).unwrap(), project_status_data.into_inner()).await;
    match operation {
        Ok(this_data) => {
            return Ok(
                ApiSuccessResponse{
                    status: "success".to_string(),
                    data: this_data
                }
            )
        },
        Err(e) => {
            return Err(e.into())
        }
    }
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
    project_status_id: String,
    project_status_usecase: &State<Arc<ProjectStatusUseCase<ImplProjectStatusRepository>>>
)
-> ApiResponse<String> {
    let operation = project_status_usecase.delete_project_status(user.id, Uuid::from_str(&project_status_id).unwrap()).await;
    match operation {
        Ok(_) => {
            return Ok(
                ApiSuccessResponse{
                    status: "success".to_string(),
                    data: "Project status deleted".to_string()
                }
            )
        },
        Err(e) => {
            return Err(e.into())
        }
    }
}

