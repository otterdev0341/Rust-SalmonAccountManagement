use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryLedgerEntryDto{
    pub id: String,
    pub chart_of_account_id: String,
    pub date: String,
    pub journal_entry_detail_id: String,
    pub debit: f64,
    pub credit: f64,
}



#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResListLedgerEntryDto {
    pub total: u32,
    pub ledger_entries: Vec<ResEntryLedgerEntryDto>,
}