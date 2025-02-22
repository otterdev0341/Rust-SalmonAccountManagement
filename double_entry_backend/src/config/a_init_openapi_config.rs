use utoipa::OpenApi;

use super::api_doc_config::{api_config::ConfigApi, api_doc_account_config::AccountApi, api_doc_auth_config::AuthApi, api_doc_company_config::CompanyApi, api_doc_customer_config::CustomerApi, api_doc_customer_contact::CustomerContactApi, api_doc_info_config::InfoApi, api_doc_location_type_config::LocationTypeApi, api_doc_project_config::ProjectApi, api_doc_project_status_config::ProjectStatusApi, api_doc_utility_config::UtilityApi};





pub fn init_openapi() -> utoipa::openapi::OpenApi {
    let register: Vec<utoipa::openapi::OpenApi> = vec![
        ConfigApi::openapi(),
        AuthApi::openapi(),
        CompanyApi::openapi(),
        UtilityApi::openapi(),
        AccountApi::openapi(),
        CustomerApi::openapi(),
        CustomerContactApi::openapi(),
        InfoApi::openapi(),
        ProjectApi::openapi(),
        LocationTypeApi::openapi(),
        ProjectStatusApi::openapi(),
    ];
    
    let mut all_api = register.into_iter();
    let mut merged_api = all_api.next().unwrap();
    
    for api in all_api {
        merged_api.merge(api);
    }
    
    merged_api
}

