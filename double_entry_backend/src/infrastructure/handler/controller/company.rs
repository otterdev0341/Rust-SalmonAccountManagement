// the company can't create by it self.
// it must be create by user and create relation within UserCompany Table


use std::sync::Arc;

use rocket::{ delete, get, post, put, routes, serde::json::Json, Route, State};

use uuid::Uuid;

use crate::{application::usecase::company_usecase::{CompanyUseCase, CompanyUseCaseError}, domain::dto::{auth_dto::AuthenticatedUser, company_dto::{ReqCreateCompanyDto, ReqUpdateCompanyDto, ResEntryCompanyDto, ResListCompanyDto, ResUpdateCompanyDto}, std_201::ResCreateSuccess, }, infrastructure::{faring::cors::options, handler::{api_response::{api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, api_success_response::{ApiCreatedResponse, ApiCreatedResponseType}, api_update_response::{ApiUpdateResponse, ApiUpdateResponseType}}, operation_status::company_error::CompanySuccess}, mysql::repositories::impl_company_repository::ImplCompanyRespository}};



pub fn company_routes() -> Vec<Route> {
    routes![
        create_company,
        edit_company,
        delete_company,
        view_company,
        view_companies,
        options
    ]
}



impl From<CompanyUseCaseError> for ApiErrorResponse {
    fn from(error: CompanyUseCaseError) -> Self {
        match error {
            CompanyUseCaseError::CompanyNotFound => ApiErrorResponse::new(404, "Company not found".to_string()),
            CompanyUseCaseError::InternalServerError => ApiErrorResponse::new(500, "Internal server error".to_string()),
            CompanyUseCaseError::ConflictingCompany => ApiErrorResponse::new(409, "Conflicting company".to_string()),
            CompanyUseCaseError::ConvertUuidError => ApiErrorResponse::new(500, "Uuid convert error".to_string()),
            CompanyUseCaseError::DeleteFailed => ApiErrorResponse::new(500, "Delete failed".to_string()),
            CompanyUseCaseError::UpdateFailed => ApiErrorResponse::new(500, "Update failed".to_string())
        }
    }
}


#[utoipa::path(
    post,
    path = "/company",
    request_body = ReqCreateCompanyDto,
    summary = "Create new company",
    description = "Create company",
    tags = ["company"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, description = "Company created"),
        (status = 400, description = "Invalid company name or description"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/", format = "json", data = "<company_data>")]
pub async fn create_company(
    user : AuthenticatedUser,
    company_data: Json<ReqCreateCompanyDto>,
    company_usecase: &State<Arc<CompanyUseCase<ImplCompanyRespository>>>
) 
-> ApiCreatedResponseType<ResCreateSuccess> {
    // validate data
    let extract = company_data.clone().into_inner();
    if extract.name.is_empty() || extract.description.is_empty() {
        return Err(ApiErrorResponse::new(400, "Invalid company name or description".to_string()));
    }

    // if not null
    let result = company_usecase.create_company( user.id ,company_data.into_inner()).await;
    match result {
        Ok(data) => {
            return Ok(ApiCreatedResponse{
                status: "success".to_string(),
                message: "Company created".to_string(),
                data: data
            });
        },
        Err(err) => {
            return Err(err.into());
        }
    }
}



#[utoipa::path(
    put,
    path = "/company/{company_id}",
    request_body = ReqUpdateCompanyDto,
    summary = "Edit company",
    description = "Edit company",
    tags = ["company"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("company_id" = String,Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "Company edited"),
        (status = 400, description = "Invalid company name or description"),
        (status = 404, description = "Company not found"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/<company_id>", format = "json", data = "<update_company>")]
pub async fn edit_company(
    user : AuthenticatedUser,
    company_id: String,
    update_company: Json<ReqUpdateCompanyDto>,
    company_usecase: &State<Arc<CompanyUseCase<ImplCompanyRespository>>>
) -> ApiUpdateResponseType<ResUpdateCompanyDto>{
    let result 
        = company_usecase.update_company(user.id, Uuid::parse_str(&company_id).unwrap(), update_company.into_inner()).await;
    match result {
        Ok(updated_data) => {
            return Ok(ApiUpdateResponse{
                status: "success".to_string(),
                message: "Company edited".to_string(),
                data: updated_data
            });
        },
        Err(err) => {
            return Err(err.into());
        }
    }
}



#[utoipa::path(
    delete,
    path = "/company/{target_id}",
    summary = "Delete company",
    description = "Delete company",
    tags = ["company"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("target_id" = String,Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "Company deleted"),
        (status = 400, description = "Invalid company id"),
        (status = 404, description = "Company not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/<target_id>")]
pub async fn delete_company(
    user : AuthenticatedUser,
    target_id: &str,
    company_usecase: &State<Arc<CompanyUseCase<ImplCompanyRespository>>>
) -> ApiResponse<String> {
    let uuid = match Uuid::parse_str(target_id) {
        Ok(id) => id,
        Err(_) => {
            return Err(ApiErrorResponse::new(400, "Invalid company id".to_string()));
        }
    };
    let result = company_usecase.delete_company(user.id, uuid).await;
    match result {
        Ok(_) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: "Company deleted".to_string()
            });
        },
        Err(err) => {
            return Err(err.into());
        }
    }
}



















#[utoipa::path(
    get,
    path = "/company/{company_id}",
    summary = "View company",
    description = "View company",
    tags = ["company"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("company_id" = String,Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "Company viewed",
            body = ResEntryCompanyDto,
            description = "Company data"
        ),
        (status = 400, description = "Invalid company id"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Company not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/<company_id>", format = "json")]
pub async fn view_company(
    user : AuthenticatedUser,
    company_usecase: &State<Arc<CompanyUseCase<ImplCompanyRespository>>>,
    company_id: String
) -> ApiResponse<ResEntryCompanyDto> {
    let result = company_usecase.get_company(user.id, Uuid::parse_str(&company_id).unwrap()).await;
    match result {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data
            });
        },
        Err(err) => {
            return Err(err.into());
        }
    }
}




#[utoipa::path(
    get,
    path = "/company",
    summary = "View companies",
    description = "View companies",
    tags = ["company"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Companies viewed",
            body = ResListCompanyDto,
            description = "List of companies of the user"
        ),
        (status = 400, description = "Invalid company id"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Company not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/", format = "json")]
pub async fn view_companies(
    user : AuthenticatedUser,
    company_usecase: &State<Arc<CompanyUseCase<ImplCompanyRespository>>>
) -> ApiResponse<ResListCompanyDto> {
    // to retrieve companies, the user must be the owner of the company
    let result = company_usecase.get_companies(user.id).await;
    match result {
        Ok(data) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data
            });
        },
        Err(err) => {
            return Err(err.into());
        }
    }
}

