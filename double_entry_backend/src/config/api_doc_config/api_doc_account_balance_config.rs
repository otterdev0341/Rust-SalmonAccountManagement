use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    paths (
        crate::infrastructure::handler::controller::account_balance::view_account_balance,
        
    ),
    components(
        schemas(
            crate::domain::dto::account_system::account_balance_dto::ResAccountBalanceDto,
            
        )
    )
)]
pub struct AccountBalanceApi;