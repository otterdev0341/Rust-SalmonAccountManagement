// the company can't create by it self.
// it must be create by user and create relation within UserCompany Table

use rocket::{post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{auth_dto::AuthenticatedUser, company_dto::{ReqCreateCompanyDto, ReqUpdateCompanyDto}, }, infrastructure::faring::cors::options};

#[allow(dead_code)]
#[allow(unused_variables)]

pub fn company_routes() -> Vec<Route> {
    routes![
        create_company, 
        options
    ]
}



// usecase create_company -> 201
// usecase edit_company -> 200
// usecase delete_company -> 200

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
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[post("/company", format = "json", data = "<company_data>")]
pub async fn create_company(
    user : AuthenticatedUser,
    company_data: Json<ReqCreateCompanyDto>
) 
-> &'static str {
    "company created"
}


#[utoipa::path(
    put,
    path = "/company",
    request_body = ReqUpdateCompanyDto,
    summary = "Edit company",
    description = "Edit company",
    tags = ["company"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Company edited"),
        (status = 400, description = "Invalid company name or description"),
        (status = 404, description = "Company not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error")
    ),
    
)]
#[put("/company", format = "json", data = "<update_company>")]
pub async fn edit_company(
    user : AuthenticatedUser,
    update_company: Json<ReqUpdateCompanyDto>
) -> &'static str {
    "company edited"
}