use rocket::{post, routes, Route};

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
    summary = "Sign up",
    description = "Sign up to the application",
    tags = ["auth"],
    responses(
        (status = 200, description = "User signed up successfully")
    )
)]
#[post("/sign-up")]
pub async fn sign_up() -> &'static str {
    "sign up"
}