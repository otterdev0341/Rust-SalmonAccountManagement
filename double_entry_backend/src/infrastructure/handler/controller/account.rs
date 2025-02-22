use rocket::{delete, get, put, routes, serde::json::Json, Route};
use crate::{domain::dto::auth_dto::{AuthenticatedUser, ReqUpdatePasswordDto, ReqUpdateUserDto, ResEntryUserDto}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};

pub fn account_routes() -> Vec<Route> {
    routes![
        delete_account,
        update_user_profile,
        change_password,
        view_profile,
        options
    ]
}


#[utoipa::path(
    delete,
    path = "/account",
    summary = "Delete account",
    description = "Delete account",
    tags = ["account"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Account deleted"),
        (status = 400, description = "Invalid account, not owner of account"),
        (status = 404, description = "Account not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[delete("/", format = "json")]
pub async fn delete_account(
    user : AuthenticatedUser
    ) 
-> ApiResponse<String> {
    
    todo!()
}






#[utoipa::path(
    put,
    path = "/account/update-profile",
    summary = "Update user profile",
    description = "Update user profile",
    tags = ["account"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User profile updated"),
        (status = 400, description = "Invalid user profile"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[put("/update-profile", format = "json", data = "<update_data>")]
pub async fn update_user_profile(
    user : AuthenticatedUser,
    update_data: Json<ReqUpdateUserDto>
    ) 
-> ApiResponse<String> {
    // must be unique and not exist in the database
    todo!()
}



#[utoipa::path(
    put,
    path = "/account/password",
    summary = "Change email",
    description = "Change email",
    tags = ["account"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Email changed"),
        (status = 400, description = "Invalid email"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[put("/password", format = "json", data = "<update_data>")]
pub async fn change_password(
    user : AuthenticatedUser,
    update_data: Json<ReqUpdatePasswordDto>
) -> ApiResponse<String> {
    
    todo!()
}





#[utoipa::path(
    get,
    path = "/account",
    summary = "View profile",
    description = "View profile",
    tags = ["account"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Profile viewed",
            body = ResEntryUserDto,
            description = "Profile data"
        ),
        (status = 404, description = "Profile not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/account")]
pub async fn view_profile(
    user : AuthenticatedUser
) -> ApiResponse<ResEntryUserDto> {
    
    todo!()
}