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
    
    #[serde(rename = "idCreated")]
    #[schema(value_type = String)]
    pub id_created: Uuid,
}

impl std::fmt::Display for ResCreateSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id_created: {}", self.id_created)
    }
}
