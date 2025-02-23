use utoipa::OpenApi;

use crate::{config::api_doc_config::api_security_addon::SecurityAddon, domain::dto::company_dto::{AddRemoveUserToCompanyDto, ReqCreateCompanyDto, ReqUpdateCompanyDto, ResCompanyRelateUserDto, ResEntryCompanyDto, ResListEntryCompanyDto}};






#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::company::create_company,
        crate::infrastructure::handler::controller::company::edit_company,
        crate::infrastructure::handler::controller::company::delete_company,
        crate::infrastructure::handler::controller::company::view_company,
        crate::infrastructure::handler::controller::company::view_companies,
        
        
    ),
    components(
        schemas(
            ReqCreateCompanyDto,
            ReqUpdateCompanyDto,
            ResEntryCompanyDto,
            ResListEntryCompanyDto,
            ResCompanyRelateUserDto,
            AddRemoveUserToCompanyDto
        )
    )
    
)]
pub struct CompanyApi;







