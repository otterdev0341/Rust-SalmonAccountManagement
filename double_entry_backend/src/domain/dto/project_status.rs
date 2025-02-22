use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "In progress",
    "description": "project is in progress",
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateProjectStatus {
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
pub struct ResEntryProjectStatus {
    pub id: String,
    pub name: String,
    pub description: String,
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
pub struct ResListProjectStatus {
    pub total: i32,
    pub statuses: Vec<ResEntryProjectStatus>,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Postponed",
    "description": "project is postponed",
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqUpdateProjectStatus {
    pub name: Option<String>,
    pub description: Option<String>,
}