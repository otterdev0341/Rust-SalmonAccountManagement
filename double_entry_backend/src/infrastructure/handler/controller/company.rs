// the company can't create by it self.
// it must be create by user and create relation within UserCompany Table


use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{auth_dto::AuthenticatedUser, company_dto::{AddRemoveUserToCompanyDto, ReqCreateCompanyDto, ReqUpdateCompanyDto, ResCompanyRelateUserDto, ResEntryCompanyDto, ResListEntryCompanyDto}, }, infrastructure::{faring::cors::options, handler::{api_response::api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, error_definition::company_error::CompanySuccess}}};



pub fn company_routes() -> Vec<Route> {
    routes![
        create_company,
        edit_company,
        delete_company,
        add_user_to_company,
        remove_user_from_company,
        view_company,
        view_companies,
        view_company_users,
        options
    ]
}



// usecase create_company -> 201
// usecase edit_company -> 200
// usecase delete_companies -> 200

// usecase add_user_to_company -> 200
// usecase remove_user_from_company -> 200

// usecase view_company -> ResEntryCompanyDto
// usecase view_companys -> ResListEntryCompanyDto
// usecase view_company_users 


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
    company_data: Json<ReqCreateCompanyDto>
) 
-> ApiResponse<String> {
    // company can't create by it self
    let result = test();
    
    
    match result {
        Ok(_) => Ok(ApiSuccessResponse::new("success", "Company created".to_string())),
        Err(_) => Err(ApiErrorResponse::new("fail to create".to_string(), "Internal server error".to_string()))
    }

    
}


fn test () -> Result<String, u8> {
    let a = 1;
    match a {
        1 => Ok("a".to_string()),
        _ => Err(1)
    }
    
}


#[utoipa::path(
    put,
    path = "/{company_id}",
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
    company_id: &str,
    update_company: Json<ReqUpdateCompanyDto>
) -> ApiResponse<String>{
    todo!()
}




#[utoipa::path(
    delete,
    path = "/company/{company_id}",
    summary = "Delete company",
    description = "Delete company",
    tags = ["company"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("company_id" = String,Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "Company deleted"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Company not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/<company_id>", format = "json")]
pub async fn delete_company(
    user : AuthenticatedUser,
    company_id: &str
) -> ApiResponse<String> {
    // only the owner of the company can delete the company
    todo!()
}









#[utoipa::path(
    post,
    path = "/company/user-company",
    request_body = AddRemoveUserToCompanyDto,
    summary = "Add user to company",
    description = "Add user to company, only the owner of the company can add user to the company",
    tags = ["user-company"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User added to company"),
        (status = 400, description = "Invalid company id or user id"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Company or user_id not found"),
        (status = 409, description = "User already added to this company"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/user-company", format = "json", data = "<company_data>")]
pub async fn add_user_to_company(
    user : AuthenticatedUser,
    company_data: Json<AddRemoveUserToCompanyDto>
) -> ApiResponse<String> {
    todo!();
}




#[utoipa::path(
    delete,
    path = "/company/user-company",
    request_body = AddRemoveUserToCompanyDto,
    summary = "Remove user from company",
    description = "Remove user from company, only the owner of the company can remove user from the company",
    tags = ["user-company"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User removed from company"),
        (status = 400, description = "Invalid company id or user id"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Company or user_id not found"),
        (status = 409, description = "User already removed from this company"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[delete("/user-company", format = "json", data = "<company_data>")]
pub async fn remove_user_from_company(
    user : AuthenticatedUser,
    company_data: Json<AddRemoveUserToCompanyDto>
) -> ApiResponse<String> {
    todo!();
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
#[get("/company/<company_id>", format = "json")]
pub async fn view_company(
    user : AuthenticatedUser,
    company_id: &str
) -> ApiResponse<ResEntryCompanyDto> {
    // to veiew company, the user must be the owner of the company
    // or the user must be the member of the company
    todo!()
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
            body = ResListEntryCompanyDto,
            description = "List of companies of the user"
        ),
        (status = 400, description = "Invalid company id"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Company not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/company", format = "json")]
pub async fn view_companies(
    user : AuthenticatedUser
) -> ApiResponse<ResListEntryCompanyDto> {
    // to retrieve companies, the user must be the owner of the company
    // or the user must be the member of the company
    todo!()
}




#[utoipa::path(
    get,
    path = "/company/user-company/{company_id}",
    summary = "View company users",
    description = "View company users",
    tags = ["user-company"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("company_id" = String,Path, description = "Company ID")
    ),
    responses(
        (status = 200, description = "Company users viewed",
            body = ResCompanyRelateUserDto,
            description = "List of users of the company"
        ),
        (status = 400, description = "Invalid company id"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Company not found"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[get("/company/user-company/<company_id>", format = "json")]
pub async fn view_company_users(
    user : AuthenticatedUser,
    company_id: &str
) -> ApiResponse<ResCompanyRelateUserDto> {
    // to retrieve company users, the user must be the owner of the company
    todo!("view company users")
}