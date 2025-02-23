use utoipa::OpenApi;

use crate::config::api_doc_config::api_security_addon::SecurityAddon;



#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::chart_of_account::create_chart_of_account,
        crate::infrastructure::handler::controller::chart_of_account::edit_chart_of_account,
        crate::infrastructure::handler::controller::chart_of_account::view_chart_of_account,
        crate::infrastructure::handler::controller::chart_of_account::view_chart_of_accounts,
        crate::infrastructure::handler::controller::chart_of_account::delete_chart_of_account,
        crate::infrastructure::handler::controller::chart_of_account::init_template,
        crate::infrastructure::handler::controller::chart_of_account::download_template
    ),
    components(
        schemas(
            crate::domain::dto::account_system::chart_of_account::ReqCreateChartOfAccountDto,
            crate::domain::dto::account_system::chart_of_account::ReqUpdateChartOfAccountDto,
            crate::domain::dto::account_system::chart_of_account::ResEntryChartOfAccountDto,
            crate::domain::dto::account_system::chart_of_account::ResListChartOfAccountDto
        )
    )
)]
pub struct ChartOfAccountApi;