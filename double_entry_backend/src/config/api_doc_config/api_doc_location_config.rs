use utoipa::OpenApi;

use crate::config::api_doc_config::api_security_addon::SecurityAddon;


#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::location::view_location,
        crate::infrastructure::handler::controller::location::view_locations,
        crate::infrastructure::handler::controller::location::edit_location,
        crate::infrastructure::handler::controller::location::delete_location,
        crate::infrastructure::handler::controller::location::create_location,
        crate::infrastructure::handler::controller::location::attach_location_to_project,
        crate::infrastructure::handler::controller::location::detach_location_from_project
    ),
    components(
        schemas(
            crate::domain::dto::location_dto::ReqCreateLocationDto,
            crate::domain::dto::location_dto::ResEntryLocationDto,
            crate::domain::dto::location_dto::ResListLocationDto,
            crate::domain::dto::location_dto::ReqUpdateLocationDto
        )
    )
)]
pub struct LocationApi;