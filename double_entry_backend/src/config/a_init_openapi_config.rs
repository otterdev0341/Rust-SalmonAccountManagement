use utoipa::OpenApi;

use super::{api_doc_auth_config::AuthApi, api_doc_company_config::CompanyApi, api_doc_utility_config::UtilityApi};



pub fn init_openapi() -> utoipa::openapi::OpenApi {
    let register: Vec<utoipa::openapi::OpenApi> = vec![
        AuthApi::openapi(),
        CompanyApi::openapi(),
        UtilityApi::openapi(),
    ];
    
    let mut all_api = register.into_iter();
    let mut merged_api = all_api.next().unwrap();
    
    for api in all_api {
        merged_api.merge(api);
    }
    
    merged_api
}

