use rocket::{delete, get, post, put, routes, serde::json::Json, Route};

use crate::{domain::dto::{account_system::journal_entry::{ReqCreateJournalEntryDto, RequpdateJournalEntryDto, ResEntryJournalEntryDto, ResListJournalEntryDto}, auth_dto::AuthenticatedUser}, infrastructure::{faring::cors::options, handler::api_response::api_response::ApiResponse}};

pub fn journal_entry_routes() -> Vec<Route> {
    
    routes![
        create_journal_entry,
        edit_journal_entry,
        view_journal_entry,
        view_journal_entries,
        delete_journal_entry,
        options
    ]
}


#[utoipa::path(
    post,
    path = "/journal-entry",
    summary = "Create journal entry",
    description = "Create journal entry",
    tags = ["journal-entry"],
    security(
        ("bearer_auth" = [])
    ),
    request_body = ReqCreateJournalEntryDto,
    responses(
        (status = 200, description = "Journal entry created"),
        (status = 400, description = "Invalid journal entry"),
        (status = 500, description = "Internal server error")
    ),
)]
#[post("/", format = "json", data = "<journal_entry_data>")]
pub async fn create_journal_entry(
    user: AuthenticatedUser,
    journal_entry_data: Json<ReqCreateJournalEntryDto>,

) 
-> ApiResponse<String> {
    todo!()
}




#[utoipa::path(
    put,
    path = "/journal-entry/<journal_entry_id>",
    summary = "Edit journal entry",
    description = "Edit journal entry",
    tags = ["journal-entry"],
    security(
        ("bearer_auth" = [])
    ),
    request_body = RequpdateJournalEntryDto,
    responses(
        (status = 200, description = "Journal entry edited"),
        (status = 400, description = "Invalid journal entry"),
        (status = 500, description = "Internal server error")
    ),
)]
#[put("/<journal_entry_id>", format = "json", data = "<journal_entry_data>")]
pub async fn edit_journal_entry(
    user: AuthenticatedUser,
    journal_entry_id: String,
    journal_entry_data: Json<RequpdateJournalEntryDto>

) -> ApiResponse<String> {
    todo!()
}




#[utoipa::path(
    get,
    path = "/journal-entry/<journal_entry_id>",
    summary = "View journal entry",
    description = "View journal entry",
    tags = ["journal-entry"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("journal_entry_id" = String,Path, description = "Journal entry id")
    ),
    responses(
        (status = 200, description = "Journal entry viewed",
            body = ResEntryJournalEntryDto,
            description = "Journal entry data"
    ),
        (status = 400, description = "Invalid journal entry"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/<journal_entry_id>", format = "json")]
pub async fn view_journal_entry(
    user: AuthenticatedUser,
    journal_entry_id: String

) -> ApiResponse<ResEntryJournalEntryDto> {
    todo!()
}



#[utoipa::path(
    get,
    path = "/journal-entry",
    summary = "View journal entries",
    description = "View journal entries",
    tags = ["journal-entry"],
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Journal entries viewed",
            body = ResListJournalEntryDto,
            description = "Journal entries data"
        ),
        (status = 400, description = "Invalid journal entries"),
        (status = 404, description = "Journal entries not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[get("/", format = "json")]
pub async fn view_journal_entries(
    user: AuthenticatedUser

) -> ApiResponse<ResListJournalEntryDto> {
    todo!()
}



#[utoipa::path(
    delete,
    path = "/journal-entry/<journal_entry_id>",
    summary = "Delete journal entry",
    description = "Delete journal entry",
    tags = ["journal-entry"],
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("journal_entry_id" = String, Path, description = "Journal entry id")
    ),
    responses(
        (status = 200, description = "Journal entry deleted"),
        (status = 400, description = "Invalid journal entry"),
        (status = 404, description = "Journal entry not found"),
        (status = 500, description = "Internal server error")
    ),
)]
#[delete("/<journal_entry_id>", format = "json")]
pub async fn delete_journal_entry(
    user: AuthenticatedUser,
    journal_entry_id: String
) -> ApiResponse<String> {
    todo!()
}