
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Invester",
    "description" : "Invest in some project",
}))]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateContactTypeDto {
    pub name: String,
    pub description: String,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "name": "Invester",
    "description": "Invest in some project",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResEntryContactTypeDto {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub created_at: String,
    pub updated_at: String
}




#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "total": 2,
    "contact_types": [
        {
            "id": "2390whflksjf0993",
            "name": "Invester",
            "description": "Invest in some project",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        },
        {
            "id": "2390whflksjf0993",
            "name": "Invester",
            "description": "Invest in some project",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        }
    ]
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResListContactTypeDto {
    pub total: u32,
    pub contact_types: Vec<ResEntryContactTypeDto>
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Customer",
    "description" : "Customer that required more information",
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateContactTypeDto{
    pub name: Option<String>,
    pub description: Option<String>,

}



#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "name": "Customer",
    "description": "Customer that required more information",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResUpdateContactTypeDto {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub updated_at: String,
}