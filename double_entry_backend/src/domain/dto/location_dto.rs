use serde::{Deserialize, Serialize};
use utoipa::ToSchema;




#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "publicName": "Otter heaven inc",
    "internalName": "Otter home",
    "lat": "6.5244",
    "long": "3.3792",
    "locationTypeId": "2390whflksjf0993",
    "description": "Otter heaven inc"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateLocationDto {
    pub public_name: String,
    pub internal_name: Option<String>,
    pub lat: Option<String>,
    pub long: Option<String>,
    pub location_type_id: Option<String>,
    pub description: Option<String>,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "publicName": "Otter heaven inc",
    "internalName": "Otter home",
    "lat": "6.5244",
    "long": "3.3792",
    "locationTypeId": "2390whflksjf0993",
    "description": "Otter heaven inc",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResEntryLocationDto {
    pub id: String,
    pub public_name: String,
    pub internal_name: Option<String>,
    pub lat: Option<String>,
    pub long: Option<String>,
    pub location_type_id: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "total": 2,
    "locations": [
        {
            "id": "2390whflksjf0993",
            "publicName": "Otter heaven inc",
            "internalName": "Otter home",
            "lat": "6.5244",
            "long": "3.3792",
            "locationTypeId": "2390whflksjf0993",
            "description": "Otter heaven inc",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        },
        {
            "id": "2390whflksjf0993",
            "publicName": "Otter heaven inc",
            "internalName": "Otter home",
            "lat": "6.5244",
            "long": "3.3792",
            "locationTypeId": "2390whflksjf0993",
            "description": "Otter heaven inc",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        }
    ]
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResListLocationDto {
    pub total: i32,
    pub locations: Vec<ResEntryLocationDto>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "publicName": "Otter heaven inc",
    "internalName": "Otter home",
    "lat": "6.5244",
    "long": "3.3792",
    "locationTypeId": "2390whflksjf0993",
    "description": "Otter heaven inc"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqUpdateLocationDto {
    pub public_name: Option<String>,
    pub internal_name: Option<String>,
    pub lat: Option<String>,
    pub long: Option<String>,
    pub location_type_id: Option<String>,
    pub description: Option<String>,
}