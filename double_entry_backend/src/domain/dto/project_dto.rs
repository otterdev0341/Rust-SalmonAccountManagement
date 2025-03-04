use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;



#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Otter heaven inc",
    "description": "use tech to grow sweet fish",
    "companyId": "2390whflksjf0993",
    "projectStatusId": ""
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqCreateProjectDto {
    pub name: String,
    pub description: String,
    #[schema(value_type = String)]
    pub company_id: Uuid,
    pub project_status_id: Option<String>,
}


#[derive(Deserialize, Serialize, ToSchema)]
#[schema(example = json!({
    "name": "all new ottter land shop inc",
    "description": "use tech to grow sweet fish",
    "projectStatusId": "2390whflksjf0993",
    "companyId": "2390whflksjf0993",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryProjectDto {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[schema(value_type = String)]
    pub company_id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    #[schema(value_type = String)]
    pub project_status_id: Option<Uuid>,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "total": 1,
    "projects": [
        {
            "id": "2390whflksjf0993",
            "name": "Otter heaven inc",
            "description": "use tech to grow sweet fish",
            "companyId": "2390whfl",
            "projectStatusId": "2390whfl",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        },
        {
            "id": "2390whflksjf0993",
            "name": "Otter heaven inc",
            "description": "use tech to grow sweet fish",
            "companyId": "2390whfl",
            "projectStatusId": "2390whfl",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        }
    ]
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResListProjectDto {
    pub total: u32,
    pub projects: Vec<ResEntryProjectDto>
}



#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "all new ottter land shop inc",
    "description": "use tech to grow sweet fish",
    "projectStatusId": "2390whflksjf0993",
    "companyId": "2390whflksjf0993",
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateProjectDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub project_status_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "all new ottter land shop inc",
    "description": "use tech to grow sweet fish",
    "projectStatusId": "2390whflksjf0993",
    "companyId": "2390whflksjf0993",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResUpdateProjectDto{
    #[schema(value_type = String)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[schema(value_type = String)]
    pub project_status_id: Option<Uuid>,
    #[schema(value_type = String)]
    pub company_id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub updated_at: String,
}
