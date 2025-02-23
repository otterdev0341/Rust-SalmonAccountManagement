use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


// this will work direcly in database, not have any api to interact


#[derive(Serialize,Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ChartOfAccountTypeDto {
    pub id: String,
    pub code: u8,
    pub definition: String,
    pub description: String,
}