use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



#[derive(Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateChartOfAccountDto {
    pub account_code: String,
    pub name: String,
    pub description: String,
    pub chart_of_account_type_id: String,
}





#[derive(Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResEntryChartOfAccountDto {
    pub id: String,
    pub account_code: String,
    pub name: String,
    pub description: String,
    pub chart_of_account_type_id: String,    
}



pub struct ResListChartOfAccountDto {
    pub total: u32,
    pub chart_of_accounts: Vec<ResEntryChartOfAccountDto>,
}



#[derive(Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqUpdateChartOfAccountDto {
    pub account_code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub chart_of_account_type_id: Option<String>,
}