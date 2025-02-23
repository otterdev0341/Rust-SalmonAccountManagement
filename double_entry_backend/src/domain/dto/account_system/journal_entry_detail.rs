use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateJournalEntryDetailDto{
    pub chart_of_account_id: String,
    pub project_id: Option<String>,
    pub description: Option<String>,
    pub amount: f64,
    pub debit: f64,
    pub credit: f64,
}


#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateJournalEntryDetailDto{
    pub chart_of_account_id: Option<String>,
    pub project_id: Option<String>,
    pub description: Option<String>,
    pub amount: Option<f64>,
    pub debit: Option<f64>,
    pub credit: Option<f64>,
}


#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryJournalEntryDetailDto{
    pub id: String,
    pub journal_entry_id: String,
    pub chart_of_account_id: String,
    pub project_id: Option<String>,
    pub description: Option<String>,
    pub amount: f64,
    pub debit: f64,
    pub credit: f64,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResListJournalEntryDetailDto{
    pub total: u32,
    pub journal_entry_details: Vec<ResEntryJournalEntryDetailDto>,
}