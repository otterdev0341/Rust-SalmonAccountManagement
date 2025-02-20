
use utoipa::OpenApi;



#[derive(OpenApi)]
#[openapi(
    paths (
        crate::infrastructure::handler::controller::auth::sign_in,
        crate::infrastructure::handler::controller::auth::sign_up
    ),
    components(
        schemas(
            crate::domain::dto::auth_dto::ReqCreateUser,
            crate::domain::dto::auth_dto::ReqSignIn,
            crate::domain::dto::auth_dto::ResSignIn
        )

    )
    
)]
pub struct AuthApi();


