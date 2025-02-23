use rocket::{get, routes, Route};

use crate::{domain::dto::{account_system::account_balance_dto::ResAccountBalanceDto, auth_dto::AuthenticatedUser}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};





pub fn account_routes() -> Vec<Route> {
    routes![
        view_account_balance,
        options
    ]
}



#[utoipa::path(
    get,
    path = "/account/balance/{account_id}",
    summary = "View account balance",
    description = "View account balance",
    tags = ["account"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("account_id" = String,Path, description = "Account ID")
    ),
    responses(
        (status = 200, description = "Account balance found",
            body = ResAccountBalanceDto,
            description = "Account balance data"
    ),
        (status = 400, description = "Invalid account balance"),
        (status = 404, description = "Account balance not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/<account_id>")]
pub async fn view_account_balance(
    user: AuthenticatedUser,
    account_id : String
) 
-> ApiResponse<ResAccountBalanceDto> {
    todo!()
}
