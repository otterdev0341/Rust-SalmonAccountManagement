use utoipa::OpenApi;

use crate::{config::api_doc_config::api_security_addon::SecurityAddon, domain::dto::contact_detail_dto::{ReqCreateContactDetailDto, ReqUpdateContactDetailDto, ResEntryContactDetailDto, ResListContactDetailDto}};



#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::contact_detail::create_contact_detail,
        crate::infrastructure::handler::controller::contact_detail::edit_contact_detail,
        crate::infrastructure::handler::controller::contact_detail::delete_contact_detail,
        crate::infrastructure::handler::controller::contact_detail::view_contact_detail,
        crate::infrastructure::handler::controller::contact_detail::view_contact_details
    ),
    components(
        schemas(
            ReqCreateContactDetailDto,
            ReqUpdateContactDetailDto,
            ResEntryContactDetailDto,
            ResListContactDetailDto,
        )
    )
)]
pub struct ContactDetailApi;