use utoipa::OpenApi;

use crate::{config::api_doc_config::api_security_addon::SecurityAddon, domain::dto::contact_dto::{ReqCreateContactDto, ReqUpdateContactDto, ResEntryContactDto, ResListContactDto}};







#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::contact::create_contact,
        crate::infrastructure::handler::controller::contact::edit_contact,
        crate::infrastructure::handler::controller::contact::delete_contact,
        crate::infrastructure::handler::controller::contact::view_contact,
        crate::infrastructure::handler::controller::contact::view_contacts
    ),
    components(
        schemas(
            ReqCreateContactDto,
            ReqUpdateContactDto,
            ResEntryContactDto,
            ResListContactDto,
        )
    )
)]
pub struct ContactApi;