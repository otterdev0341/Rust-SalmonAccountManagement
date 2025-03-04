use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "infoId": "2390whflksjf0993",
    "projectId": "2390whflksjf0993",
    "message": "info added to project"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ResAddInfoToProjectDto {
    
    pub message: String,
    #[schema(value_type = String)]
    pub info_id: Uuid,
    #[schema(value_type = String)]
    pub project_id: Uuid,

}