use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


// this will create with Charte of account is created
// Active model impl BeforeInsert


#[derive(Serialize,Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResAccountBalanceDto {
    pub id: String,
    pub account_id: String,
    pub balance: f64,
    pub last_updated: String,
}