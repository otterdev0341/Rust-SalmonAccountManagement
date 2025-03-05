use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{info_dto::ResEntryInfoDto, project_dto::ResEntryProjectDto};

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "infoId": "2390whflksjf0993",
    "projectId": "2390whflksjf0993",
    "message": "info added to project"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResInfoProjectDto {
    
    pub message: String,
    #[schema(value_type = String)]
    pub info_id: Uuid,
    #[schema(value_type = String)]
    pub project_id: Uuid,

}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "infoId": "2390whflksjf0993",
    "projectId": "2390whflksjf0993",
    "message": "info removed from project"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResInfosOfProjectDto{
    pub total: u32,
    pub infos: Vec<ResEntryInfoDto>
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "infoId": "2390whflksjf0993",
    "projectId": "2390whflksjf0993",
    "message": "project removed from info"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResProjectsOfInfoDto{
    pub total: u32,
    pub projects: Vec<ResEntryProjectDto>
}