use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    paths (
        crate::infrastructure::handler::controller::project::view_project,
        crate::infrastructure::handler::controller::project::view_projects,
        crate::infrastructure::handler::controller::project::edit_project,
        crate::infrastructure::handler::controller::project::delete_project,
        crate::infrastructure::handler::controller::project::create_project
    ),
    components(
        schemas(
            crate::domain::dto::project_dto::ReqCreateProjectDto,
            crate::domain::dto::project_dto::ResEntryProjectDto,
            crate::domain::dto::project_dto::ResListProjectDto
        )
    )
)]
pub struct ProjectApi;