use rocket::{delete, get, post, put, routes, serde::json::Json, Data, Route};

use crate::{domain::dto::{account_system::chart_of_account::{ReqCreateChartOfAccountDto, ReqUpdateChartOfAccountDto, ResEntryChartOfAccountDto, ResListChartOfAccountDto}, auth_dto::AuthenticatedUser}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};



pub fn chart_of_account_routes() -> Vec<Route> {
    routes![
        create_chart_of_account,
        edit_chart_of_account,
        view_chart_of_account,
        view_chart_of_accounts,
        delete_chart_of_account,
        init_template,
        download_template,
        options
    ]
}




#[utoipa::path(
    post,
    path = "/chart-of-account",
    summary = "Create chart of account",
    description = "Create chart of account",
    tags = ["chart-of-account"],
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateChartOfAccountDto,
    responses(
        (status = 200, description = "Chart of account created"),
        (status = 400, description = "Invalid chart of account"),
        (status = 500, description = "Internal server error")
    ),
)]
#[post("/", format = "json", data = "<chart_of_account_data>")]
pub async fn create_chart_of_account(
    user : AuthenticatedUser,
    chart_of_account_data: Json<ReqCreateChartOfAccountDto>
    )
-> ApiResponse<String> {
    todo!()
}





#[utoipa::path(
    put,
    path = "/chart-of-account",
    summary = "Edit chart of account",
    description = "Edit chart of account",
    tags = ["chart-of-account"],
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqUpdateChartOfAccountDto,
    responses(
        (status = 200, description = "Chart of account updated"),
        (status = 400, description = "Invalid chart of account"),
        (status = 500, description = "Internal server error")
    ),
)]
#[put("/", format = "json", data = "<chart_of_account_data>")]
pub async fn edit_chart_of_account(
    user : AuthenticatedUser,
    chart_of_account_data: Json<ReqUpdateChartOfAccountDto>
    )
-> ApiResponse<String> {
    todo!()
}



#[utoipa::path(
    get,
    path = "/chart-of-account/{chart_of_account_id}",
    summary = "View chart of account",
    description = "View chart of account",
    tags = ["chart-of-account"],
    params(
        ("chart_of_account_id" = String, Path, description = "Chart of account id")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Chart of account found",
            body = ResEntryChartOfAccountDto, 
            description = "Chart of account found"
        ),
        (status = 404, description = "Chart of account not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/<chart_of_account_id>", format = "json")]
pub fn view_chart_of_account(
    user: AuthenticatedUser,
    chart_of_account_id: String
) -> ApiResponse<ResEntryChartOfAccountDto> {
    todo!()
}





#[utoipa::path(
    get,
    path = "/chart-of-account",
    summary = "View chart of accounts",
    description = "View chart of accounts",
    tags = ["chart-of-account"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Chart of accounts found",
            body = ResListChartOfAccountDto, 
            description = "Chart of accounts found"
        ),
        (status = 404, description = "Chart of accounts not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/", format = "json")]
pub fn view_chart_of_accounts(
    user: AuthenticatedUser
) -> ApiResponse<ResListChartOfAccountDto> {
    todo!()
}



#[utoipa::path(
    delete,
    path = "/chart-of-account/{chart_of_account_id}",
    summary = "Delete chart of account",
    description = "Delete chart of account",
    tags = ["chart-of-account"],
    params(
        ("chart_of_account_id" = String, Path, description = "Chart of account id")
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Chart of account deleted"),
        (status = 404, description = "Chart of account not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[delete("/<chart_of_account_id>", format = "json")]
pub async fn delete_chart_of_account(
    user : AuthenticatedUser,
    chart_of_account_id: String
    )
-> ApiResponse<String> {
    todo!()
}



#[utoipa::path(
    post,
    path = "/chart-of-account/init-template",
    summary = "Init template",
    description = "Init template",
    tags = ["chart-of-account"],
    request_body(
        content_type = "text/csv",
        description = "CSV file containing template data",
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Template created"),
        (status = 500, description = "Internal server error")
    ),
)]
#[post("/init-template", format = "text/csv", data = "<template_data>")]
pub async fn init_template(
    user : AuthenticatedUser,
    template_data: Data<'_>
) -> ApiResponse<String> {
    todo!()
}



#[utoipa::path(
    get,
    path = "/chart-of-account/template",
    summary = "Download template",
    description = "Download template",
    tags = ["chart-of-account"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Template downloaded",
            content_type = "text/csv",
            description = "Template downloaded"
        ),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/template", format = "text/csv")]
pub async fn download_template(
    user : AuthenticatedUser,
) -> (rocket::http::ContentType, Vec<u8>)   {
    todo!()
}