use serde::{Deserialize, Serialize};
use utoipa::ToSchema;



#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "projectId": "2390whflksjf0993",
    "userId": "2390whflksjf0993",
    "title": "Otter heaven inc",
    "content": "use tech to grow sweet fish",
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateInfoDto {
    pub project_id : String,
    pub user_id: Option<String>,
    pub title: String,
    pub content: String,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "projectId": "2390whflksjf0993",
    "userId": "2390whflksjf0993",
    "title": "Otter heaven inc",
    "content": "use tech to grow sweet fish",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryInfoDto {
    pub id: String,
    pub project_id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "total": 2,
    "infos": [
        {
            "id": "2390whflksjf0993",
            "project_id": "2390whflksjf0993",
            "userId": "2390whflksjf0993",
            "title": "Otter heaven inc",
            "content": "use tech to grow sweet fish",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        },
        {
            "id": "2390whflksjf0993",
            "projectId": "2390whflksjf0993",
            "userId": "2390whflksjf0993",
            "title": "Otter heaven inc",
            "content": "use tech to grow sweet fish",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        }
    ]
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct RestListInfoDto {
    pub total: i32,
    pub infos: Vec<ResEntryInfoDto>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "title": "Otter heaven inc",
    "content": "use tech to grow sweet fish",
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateInfoDto {
    pub title: Option<String>,
    pub content: Option<String>,
}

