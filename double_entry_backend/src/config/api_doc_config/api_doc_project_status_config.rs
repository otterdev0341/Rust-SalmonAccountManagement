use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    paths (
        crate::infrastructure::handler::controller::project_status::view_project_status,
        crate::infrastructure::handler::controller::project_status::view_project_statuses,
        crate::infrastructure::handler::controller::project_status::edit_project_status,
        crate::infrastructure::handler::controller::project_status::delete_project_status,
        crate::infrastructure::handler::controller::project_status::create_project_status
    ),
    components(
        schemas(
            crate::domain::dto::project_status::ReqCreateProjectStatusDto,
            crate::domain::dto::project_status::ResEntryProjectStatusDto,
            crate::domain::dto::project_status::ResListProjectStatusDto,
            crate::domain::dto::project_status::ReqUpdateProjectStatusDto
        )
    )
)]
pub struct ProjectStatusApi;