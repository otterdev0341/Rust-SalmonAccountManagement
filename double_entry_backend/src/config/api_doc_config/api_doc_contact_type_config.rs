use utoipa::OpenApi;

use crate::config::api_doc_config::api_security_addon::SecurityAddon;



#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::contact_type::create_contact_type,
        crate::infrastructure::handler::controller::contact_type::update_contact_type,
        crate::infrastructure::handler::controller::contact_type::delete_contact_type,
        crate::infrastructure::handler::controller::contact_type::view_contact_type,
        crate::infrastructure::handler::controller::contact_type::view_contact_types
    ),
    components(
        schemas(
            crate::domain::dto::contact_type_dto::ReqCreateContactTypeDto,
            crate::domain::dto::contact_type_dto::ReqUpdateContactTypeDto,
            crate::domain::dto::contact_type_dto::ResEntryContactTypeDto,
            crate::domain::dto::contact_type_dto::ResListContactTypeDto,
        )
    )
)]
pub struct ContactTypeApi;