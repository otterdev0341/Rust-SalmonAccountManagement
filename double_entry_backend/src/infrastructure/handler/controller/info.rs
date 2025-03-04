use std::{str::FromStr, sync::Arc};

use rocket::{delete, get, post, put, routes, serde::json::Json, Route, State};
use serde::de;
use utoipa::openapi::info;
use uuid::Uuid;
use crate::{application::usecase::info_usecase::{InfoUseCase, InfoUseCaseError}, domain::dto::{auth_dto::AuthenticatedUser, info_dto::{ReqCreateInfoDto, ReqUpdateInfoDto, ResEntryInfoDto, ResListInfoDto, ResUpdateInfoDto}, std_201::ResCreateSuccess}, infrastructure::{faring::cors::options, handler::api_response::{api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, api_success_response::{ApiCreatedResponse, ApiCreatedResponseType}, api_update_response::{ApiUpdateResponse, ApiUpdateResponseType}}, mysql::repositories::impl_info_repository::ImplInfoRepository}};


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

impl From<InfoUseCaseError> for ApiErrorResponse{
    fn from(error: InfoUseCaseError) -> Self {
        match error {
            InfoUseCaseError::InfoNotFound => ApiErrorResponse::new(404, "Info not found".to_string()),
            InfoUseCaseError::InternalServerError => ApiErrorResponse::new(500, "Internal server error".to_string()),
            InfoUseCaseError::ConflictingInfo => ApiErrorResponse::new(409, "Conflicting info".to_string()),
            InfoUseCaseError::DeleteFailed => ApiErrorResponse::new(500, "Delete failed".to_string()),
            InfoUseCaseError::UpdateFailed => ApiErrorResponse::new(500, "Update failed".to_string())
        }
    }
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
    info_data: Json<ReqCreateInfoDto>,
    info_usecase: &State<Arc<InfoUseCase<ImplInfoRepository>>>
    ) 
-> ApiCreatedResponseType<ResCreateSuccess> {
    if info_data.title.is_empty() || info_data.content.is_empty() {
        return Err(ApiErrorResponse::new(400, "Invalid title or content".to_string()));
    }
    let operation = info_usecase.create_info(user.id, info_data.into_inner()).await;
    match operation {
        Ok(data) => {
            return Ok(ApiCreatedResponse {
                status: "success".to_string(),
                message: "Project info created".to_string(),
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
    info_id: String,
    info_usecase: &State<Arc<InfoUseCase<ImplInfoRepository>>>
) -> ApiResponse<ResEntryInfoDto> {
    let operation = info_usecase.get_info(user.id, Uuid::from_str(&info_id).unwrap()).await;
    match operation {
        Ok(data) => {
            return Ok(
                ApiSuccessResponse{
                    status: "success".to_string(),
                    data: data
                }
            );
        },
        Err(err) => {
            return Err(err.into());
        }
    }
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
            body = ResListInfoDto,
            description = "Project infos found"
        ),
        (status = 404, description = "Project infos not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/info", format = "json")]
pub async fn view_project_infos(
    user: AuthenticatedUser,
    info_usecase: &State<Arc<InfoUseCase<ImplInfoRepository>>>
) -> ApiResponse<ResListInfoDto> {
    let operation = info_usecase.get_infos(user.id).await;
    match operation {
        Ok(data) => {
            return Ok(
                ApiSuccessResponse{
                    status: "success".to_string(),
                    data: data
                }
            );
        },
        Err(err) => {
            return Err(err.into());
        }
    }
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
    info_data: Json<ReqUpdateInfoDto>,
    info_usecase: &State<Arc<InfoUseCase<ImplInfoRepository>>>
    ) 
-> ApiUpdateResponseType<ResUpdateInfoDto> {
    let operation = info_usecase.update_info(user.id, Uuid::from_str(&info_id).unwrap(), info_data.into_inner()).await;
    match operation {
        Ok(data) => {
            return Ok(ApiUpdateResponse {
                status: "success".to_string(),
                message: "Project info edited".to_string(),
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
    info_id: String,
    info_usecase: &State<Arc<InfoUseCase<ImplInfoRepository>>>
) 
-> ApiResponse<String> {
    let operation = info_usecase.delete_info(user.id, Uuid::from_str(&info_id).unwrap()).await;
    match operation {
        Ok(_) => {
            return Ok(
                ApiSuccessResponse{
                    status: "success".to_string(),
                    data: "Project info deleted".to_string()
                }
            );
        },
        Err(err) => {
            return Err(err.into());
        }
    }
}

