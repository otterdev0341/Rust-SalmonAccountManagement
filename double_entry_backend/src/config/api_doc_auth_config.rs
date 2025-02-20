
use utoipa::OpenApi;



#[derive(OpenApi)]
#[openapi(
    paths (
        crate::infrastructure::handler::controller::auth::sign_in,
        crate::infrastructure::handler::controller::auth::sign_up
    ),
    components(
        schemas(
            crate::domain::dto::user::ReqCreateUser
        )

    )
    
)]
pub struct AuthApi();


