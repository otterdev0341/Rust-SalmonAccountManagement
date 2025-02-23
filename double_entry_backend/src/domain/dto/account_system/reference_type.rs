use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateReferenceTypeDto {
    pub name: String,
    pub description: Option<String>,
}




#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryReferenceTypeDto{
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}




#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResListReferenceTypeDto {
    pub total: u32,
    pub reference_types: Vec<ResEntryReferenceTypeDto>,
}




#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateReferenceTypeDto {
    pub name: Option<String>,
    pub description: Option<String>,
}