use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::journal_entry_detail::{ReqCreateJournalEntryDetailDto, ReqUpdateJournalEntryDetailDto, ResEntryJournalEntryDetailDto};



#[derive(Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateJournalEntryDto {
    pub date: String,
    pub description: Option<String>,
    pub reference_type_id: String,
    pub contact_id: String,
    pub data: Vec<ReqCreateJournalEntryDetailDto>,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct RequpdateJournalEntryDto {
    pub date: Option<String>,
    pub description: Option<String>,
    pub reference_type_id: Option<String>,
    pub contact_id: Option<String>,
    pub data: Vec<ReqUpdateJournalEntryDetailDto>
}


#[derive(Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResEntryJournalEntryDto {
    pub id: String,
    pub date: String,
    pub description: Option<String>,
    pub reference_type_id: String,
    pub contact_id: String,
    pub data: Vec<ResEntryJournalEntryDetailDto>,
    pub created_at: String,
    pub updated_at: String,
}



#[derive(Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResListJournalEntryDto {
    pub total: u32,
    pub journal_entries: Vec<ResEntryJournalEntryDto>,
}