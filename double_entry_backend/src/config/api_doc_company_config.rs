use utoipa::{openapi::security::{HttpAuthScheme, SecurityScheme}, Modify, OpenApi};

use crate::{config::api_security_addon::SecurityAddon, domain::dto::{auth_dto::ResEntryUserDto, company_dto::{ReqCreateCompanyDto, ReqUpdateCompanyDto, ResCompanyRelateUserDto, ResEntryCompanyDto, ResListEntryCompanyDto}}};




#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::company::create_company,
        crate::infrastructure::handler::controller::company::edit_company,
        
        
    ),
    components(
        schemas(
            ReqCreateCompanyDto,
            ReqUpdateCompanyDto,
            ResEntryCompanyDto,
            ResListEntryCompanyDto,
            ResCompanyRelateUserDto,
        )

    )
    
)]
pub struct CompanyApi;







