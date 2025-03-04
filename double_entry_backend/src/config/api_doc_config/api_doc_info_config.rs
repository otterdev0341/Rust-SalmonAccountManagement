use utoipa::OpenApi;

use crate::{config::api_doc_config::api_security_addon::SecurityAddon, domain::dto::info_dto::{ReqCreateInfoDto, ReqUpdateInfoDto, ResEntryInfoDto, ResListInfoDto}};


#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::info::view_project_info,
        crate::infrastructure::handler::controller::info::view_project_infos,
        crate::infrastructure::handler::controller::info::edit_project_info,
        crate::infrastructure::handler::controller::info::delete_project_info,
        crate::infrastructure::handler::controller::info::create_project_info
    ),
    components(
        schemas(
            ReqCreateInfoDto,
            ReqUpdateInfoDto,
            ResEntryInfoDto,
            ResListInfoDto
        )
    )
)]
pub struct InfoApi;