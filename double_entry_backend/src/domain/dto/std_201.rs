use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "idCreated": "2390whflksjf0993",
    
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResCreateSuccess{
    
    #[schema(value_type = String)]
    pub id_created: Uuid,
}


