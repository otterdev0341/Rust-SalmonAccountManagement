use utoipa::OpenApi;



#[derive(OpenApi)]
#[openapi(
    paths (
        crate::infrastructure::handler::controller::utility::server_check,
        crate::infrastructure::handler::controller::utility::database_check
    )
)]
pub struct UtilityApi;