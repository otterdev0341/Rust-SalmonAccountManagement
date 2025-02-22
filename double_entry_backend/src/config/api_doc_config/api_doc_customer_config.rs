use utoipa::OpenApi;

use crate::{config::api_doc_config::api_security_addon::SecurityAddon, domain::dto::customer_dto::{ReqCreateCustomerDto, ReqUpdateCustomerDto, ResEntryCustomerDto}};





#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::customer::create_customer,
        crate::infrastructure::handler::controller::customer::edit_customer,
        crate::infrastructure::handler::controller::customer::delete_customer,
        crate::infrastructure::handler::controller::customer::view_customer,
        crate::infrastructure::handler::controller::customer::view_customers
    ),
    components(
        schemas(
            ReqCreateCustomerDto,
            ReqUpdateCustomerDto,
            ResEntryCustomerDto
        )
    )
)]
pub struct CustomerApi;