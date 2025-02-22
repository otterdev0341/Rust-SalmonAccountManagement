use utoipa::OpenApi;

use crate::config::api_doc_config::api_security_addon::SecurityAddon;


#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
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
            crate::domain::dto::project_dto::ResListProjectDto,
            crate::domain::dto::project_dto::ReqUpdateProjectDto
        )
    )
)]
pub struct ProjectApi;