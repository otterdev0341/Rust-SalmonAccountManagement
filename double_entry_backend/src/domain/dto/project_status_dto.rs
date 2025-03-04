use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;



#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "In progress",
    "description": "project is in progress",
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateProjectStatusDto {
    pub name: String,
    pub description: String,
}



#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "name": "Inprogress",
    "description": "project is in progress",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryProjectStatusDto {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "total": 1,
    "projectStatuses": [
        {
            "id": "2390whflksjf0993",
            "name": "Inprogress",
            "description": "project is in progress",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        },
        {
            "id": "2390whflksjf0993",
            "name": "Urgent Review",
            "description": "This project needs urgent review",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        }
    ]
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResListProjectStatusDto {
    pub total: u32,
    pub statuses: Vec<ResEntryProjectStatusDto>,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Postponed",
    "description": "project is postponed",
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqUpdateProjectStatusDto {
    pub name: Option<String>,
    pub description: Option<String>,
}



#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "name": "Postponed",
    "description": "project is postponed",
    "userId": "2390whflksjf0993",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResUpdateProjectStatusDto {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub updated_at: String,
}