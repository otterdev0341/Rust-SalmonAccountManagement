use rocket::{get, routes, Route};


use crate::{config::db_config::DBConfig, infrastructure::{handler::api_response::api_response::{ApiErrorResponse, ApiResponse, ApiSuccessResponse}, mysql::connection::mysql_connect::mysql_connec}};






pub fn utility_routes() -> Vec<Route> {
    routes![
        database_check,
        server_check,
    ]
}



#[utoipa::path(
    get,
    path = "/server_status",
    summary = "Check server status",
    description = "Check server status",
    tags = ["utility"],
    responses(
        (status = 200, description = "Server is running"),
    ),
)]
#[get("/server_status")]
pub async fn server_check() -> ApiResponse<String> {
    Ok(ApiSuccessResponse::new("200", "Server is running".to_string()))
}


#[utoipa::path(
    get,
    path = "/database_status",
    summary = "Check database status",
    description = "Check database status",
    tags = ["utility"],
    responses(
        (status = 200, description = "Database is running"),
        (status = 500, description = "Database is not running"),
    ),
)]
#[get("/database_status")]
pub async fn database_check() -> ApiResponse<String> {
    let databse_config = DBConfig::default();
    let db_connection = mysql_connec(&databse_config).await;
    match db_connection {
        Ok(_) => (Ok(ApiSuccessResponse::new("200", "Database is running".to_string()))),
        Err(_) => return Err(ApiErrorResponse::new("500".to_string(), "Database is not running".to_string()))
    }

    
}