use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



#[derive(Serialize,Deserialize,ToSchema)]
#[schema(example = json!({
    "name": "Otter heaven inc",
    "description": "use tech to grow sweet fish"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateLocationType {
    pub name: String,
    pub description: Option<String>,
}


#[derive(Serialize,Deserialize,ToSchema)]
#[schema(example = json!({
    "name": "Pool Villa",
}))]
#[serde(crate = "rocket::serde")]
pub struct ResEntryLocationType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Serialize,Deserialize,ToSchema)]
#[schema(example = json!({
    "total": 2,
    "location_types": [
        {
            "id": "2390whflksjf0993",
            "name": "Town home",
            "description": "town home",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        },
        {
            "id": "2390whflksjf0993",
            "name": "Pool Villa",
            "description": "Villa with Pool",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
    }
    ]}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResListLocationType {
    pub total: i64,
    pub location_types: Vec<ResEntryLocationType>,
}



#[derive(Serialize,Deserialize,ToSchema)]
#[schema(example = json!({
    "name": "House",
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqUpdateLocationTypeDto {
    pub name: Option<String>,
    pub description: Option<String>,
}