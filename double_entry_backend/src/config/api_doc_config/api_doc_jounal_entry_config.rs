use utoipa::OpenApi;

use crate::config::api_doc_config::api_security_addon::SecurityAddon;


#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    modifiers(&SecurityAddon),
    paths (
        crate::infrastructure::handler::controller::journal_entry::create_journal_entry,
        crate::infrastructure::handler::controller::journal_entry::edit_journal_entry,
        crate::infrastructure::handler::controller::journal_entry::view_journal_entry,
        crate::infrastructure::handler::controller::journal_entry::view_journal_entries,
        crate::infrastructure::handler::controller::journal_entry::delete_journal_entry
    ),
    components(
        schemas(
            crate::domain::dto::account_system::journal_entry::ReqCreateJournalEntryDto,
            crate::domain::dto::account_system::journal_entry::RequpdateJournalEntryDto,
            crate::domain::dto::account_system::journal_entry::ResEntryJournalEntryDto,
            crate::domain::dto::account_system::journal_entry::ResListJournalEntryDto
        )
    )
)]
pub struct JournalEntryApi;