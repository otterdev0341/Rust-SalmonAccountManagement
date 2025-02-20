use rocket::{post, routes, serde::json::Json, Route};

use crate::domain::dto::auth_dto::{ReqCreateUser, ReqSignIn, ResSignIn};

#[allow(dead_code)]

// this for register at a_init_routes.rs
pub fn auth_routes() -> Vec<Route> {
    routes![
        sign_up, 
        sign_in]
}




#[utoipa::path(
    post,
    path = "/sign-in",
    request_body = ReqSignIn, 
    summary = "Sign in",
    description = "Sign in to the application",
    tags = ["auth"],
    responses(
        (status = 200, description = "User signed in successfully"),
        (status = 400, description = "Invalid email or password"),
        (status = 500, description = "Internal server error"),
        (status = 404, description = "Email not found"),
        (status = 401, description = "Incorrect email or password")
    )
)]
#[post("/sign-in", format = "json", data = "<req_sign_in>")]
pub async fn sign_in(req_sign_in: Json<ReqSignIn>) -> &'static str {
    "sign in"
}


#[utoipa::path(
    post,
    path = "/sign-up",
    request_body = ReqCreateUser,
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
#[post("/sign-up", format = "json", data = "<req_sign_in>")]
pub async fn sign_up(req_sign_in: Json<ReqCreateUser>) -> Json<ResSignIn> {
    
    Json(ResSignIn {
        token: "token".to_string(),
    })
}

