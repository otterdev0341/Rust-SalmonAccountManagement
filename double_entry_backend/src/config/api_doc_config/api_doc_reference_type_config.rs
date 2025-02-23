use utoipa::OpenApi;

use crate::config::api_doc_config::api_security_addon::SecurityAddon;



#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::reference_type::create_reference_type,
        crate::infrastructure::handler::controller::reference_type::edit_reference_type,
        crate::infrastructure::handler::controller::reference_type::view_reference_type,
        crate::infrastructure::handler::controller::reference_type::view_reference_types,
        crate::infrastructure::handler::controller::reference_type::delete_reference_type
    ),
    components(
        schemas(
            crate::domain::dto::account_system::reference_type_dto::ReqCreateReferenceTypeDto,
            crate::domain::dto::account_system::reference_type_dto::ReqUpdateReferenceTypeDto,
            crate::domain::dto::account_system::reference_type_dto::ResEntryReferenceTypeDto,
            crate::domain::dto::account_system::reference_type_dto::ResListReferenceTypeDto
            
        )
    )
)]
pub struct ReferenceTypeApi;