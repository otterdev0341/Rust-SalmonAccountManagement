use rocket::{post, routes, Route};

use crate::domain::dto::user::ReqCreateUser;

// this for register at a_init_routes.rs
pub fn auth_routes() -> Vec<Route> {
    routes![
        sign_up, 
        sign_in]
}





#[utoipa::path(
    post,
    path = "/sign-in",
    summary = "Sign in",
    description = "Sign in to the application",
    tags = ["auth"],
    responses(
        (status = 200, description = "User signed in successfully")
    )
)]
#[post("/sign-in")]
pub async fn sign_in() -> &'static str {
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
#[post("/sign-up")]
pub async fn sign_up() -> &'static str {
    "sign up"
}