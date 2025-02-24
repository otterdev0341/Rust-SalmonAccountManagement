use std::{f32::consts::E, sync::Arc};

use rocket::{http::Status, post, routes, serde::json::Json, Route, State};

use crate::{application::usecase::auth_usecase::AuthUseCase, domain::dto::auth_dto::{ReqCreateUserDto, ReqSignInDto, ResSignInDto}, infrastructure::{faring::cors::options, handler::{api_response::api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, operation_status::auth_error::{CreateUserError, SignInError}, validate_util::auth_validate::check_req_create_user_dto}, mysql::repositories::impl_auth_repository::ImplAuthRepository}};

#[allow(dead_code)]


// this for register at a_init_routes.rs
// assign options to allow parse json body in to the request
pub fn auth_routes() -> Vec<Route> {
    routes![
        sign_up, 
        sign_in,
        options
    ]
}




#[utoipa::path(
    post,
    path = "/auth/sign-in",
    request_body = ReqSignInDto,
    summary = "Sign in",
    description = "Sign in to the application",
    tags = ["auth"],
    responses(
        (status = 200, description = "User signed in successfully",
            body = ResSignInDto,
            description = "Token that return to user",
        ),
        (status = 400, description = "Invalid email or password"),
        (status = 500, description = "Internal server error"),
        (status = 404, description = "Email not found"),
        (status = 401, description = "Incorrect email or password")
    )
)]
#[post("/sign-in", format = "json", data = "<req_sign_in>")]
pub async fn sign_in(
    req_sign_in: Json<ReqSignInDto>,
    auth_usecase: &State<Arc<AuthUseCase<ImplAuthRepository>>>
) 
-> ApiResponse<ResSignInDto> {
    
    let result = auth_usecase.sign_in(req_sign_in.into_inner()).await;
    match result {
        Ok(token) => {
            return Ok(ApiSuccessResponse{
                status: "success".to_string(),
                data: token
            });
        },
        Err(_err) => {
            
                return Err(ApiErrorResponse::new(400, "Invalid email or password".to_string()))
            
        }
    }
}    
    



#[utoipa::path(
    post,
    path = "/auth/sign-up",
    request_body = ReqCreateUserDto,
    summary = "Sign up",
    description = "Sign up to the application",
    tags = ["auth"],
    responses(
        (status = 201, description = "User signed up successfully"),
        (status = 409, description = "Username or email already exists"),
        (status = 400, description = "Invalid username, email, password, first name, or last name"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/sign-up", format = "json", data = "<req_sign_up>")]
pub async fn sign_up(
    auth_usecase: &State<Arc<AuthUseCase<ImplAuthRepository>>>,
    req_sign_up: Json<ReqCreateUserDto>) 
    
-> ApiResponse<String> {
    
    let req_sign_up_clone = req_sign_up.into_inner().clone();
    let check_data = check_req_create_user_dto(&req_sign_up_clone);
    match check_data {
        Ok(_) => (),
        Err(e) => return Err(ApiErrorResponse::new(200, e.to_string()))
    }
    let result = auth_usecase.create_user(req_sign_up_clone).await;
    
    if let Err(err) = result {
        match err {
            CreateUserError::UsernameAlreadyExists => return Err(ApiErrorResponse::new(409, "Username already exists".to_string())),
            CreateUserError::EmailAlreadyExists => return Err(ApiErrorResponse::new(409, "Email already exists".to_string())),
            CreateUserError::InternalServerError => return Err(ApiErrorResponse::new(500, "Internal server error".to_string())),
            _ => return Err(ApiErrorResponse::new(400, "Invalid username, email, password, first name, or last name".to_string()))
        }
    }

    
    Ok(ApiSuccessResponse::new("success","Create user succesfull".to_string()))
    

    
}

