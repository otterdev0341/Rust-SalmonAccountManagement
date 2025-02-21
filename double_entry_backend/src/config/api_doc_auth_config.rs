
use utoipa::OpenApi;



#[derive(OpenApi)]
#[openapi(
    info(
        title = "Salmon Account Management API",
        version = "0.1.0",
        description = "API for Entry Project"
    ),
    paths (
        crate::infrastructure::handler::controller::auth::sign_in,
        crate::infrastructure::handler::controller::auth::sign_up
    ),
    components(
        schemas(
            crate::domain::dto::auth_dto::ReqCreateUserDto,
            crate::domain::dto::auth_dto::ReqSignInDto,
            crate::domain::dto::auth_dto::ResSignInDto,
        )

    )
    
)]
pub struct AuthApi;


