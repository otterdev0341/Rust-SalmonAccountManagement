use utoipa::OpenApi;

use crate::config::api_doc_config::api_security_addon::SecurityAddon;

#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::account::delete_account,
        crate::infrastructure::handler::controller::account::update_user_profile,
        crate::infrastructure::handler::controller::account::change_password,
        crate::infrastructure::handler::controller::account::view_profile
    ),
    components(
        schemas(
            crate::domain::dto::auth_dto::ReqUpdatePasswordDto,
            crate::domain::dto::auth_dto::ReqUpdateUserDto,
            crate::domain::dto::auth_dto::ResEntryUserDto
        )
    )
)]
pub struct AccountApi;