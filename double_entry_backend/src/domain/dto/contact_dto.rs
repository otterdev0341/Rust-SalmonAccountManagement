use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Otter heaven inc",
    "companyId": "2390whflksjf0993",
    "contact_type_id": "2390whflksjf0993"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateContactDto {
    pub name: String,
    pub company_id: String,
    pub contact_type_id: Option<String>
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "all new ottter land shop inc",
    "contactTypeId": "2390whflksjf0993"
}))]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateContactDto {
    pub name: Option<String>,
    pub contact_type_id: Option<String>
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "name": "Otter heaven inc",
    "companyId": "2390whflksjf0993",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResEntryContactDto {
    pub id: String,
    pub name: String,
    pub company_id: String,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "total": 1,
    "customers": [
        {
            "id": "2390whflksjf0993",
            "name": "Otter heaven inc",
            "companyId": "2390whfl",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        },
        {
            "id": "2390whflksjf0993",
            "name": "Otter heaven inc",
            "companyId": "2390whfl",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        }
    ]
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResListContactDto {
    pub total: i32,
    pub customers: Vec<ResEntryContactDto>,
}