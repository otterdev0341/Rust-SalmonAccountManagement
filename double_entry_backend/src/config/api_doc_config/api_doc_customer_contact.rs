use utoipa::OpenApi;

use crate::config::api_doc_config::api_security_addon::SecurityAddon;



#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::customer_contact::create_customer_contact,
        crate::infrastructure::handler::controller::customer_contact::edit_customer_contact,
        crate::infrastructure::handler::controller::customer_contact::delete_customer_contact,
        crate::infrastructure::handler::controller::customer_contact::view_customer_contact,
        crate::infrastructure::handler::controller::customer_contact::view_customer_contacts
    ),
    components(
        schemas(
            crate::domain::dto::customer_contact_dto::ReqCreateCustomerContactDto,
            crate::domain::dto::customer_contact_dto::ReqUpdateCustomerContactDto,
            crate::domain::dto::customer_contact_dto::ResListCustomerContactDto,
            crate::domain::dto::customer_contact_dto::ResEntryCustomerContactDto,
        )
    )
)]
pub struct CustomerContactApi;