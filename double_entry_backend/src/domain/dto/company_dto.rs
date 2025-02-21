use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::auth_dto::ResEntryUserDto;






#[derive(Deserialize,Serialize,ToSchema)]
#[schema(example = json!({
    "name": "Otter heaven inc",
    "description": "use tech to grow sweet fish"
}))]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateCompanyDto {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize,Serialize,ToSchema)]
#[schema(example = json!({
    "name": "Otter heaven holding group",
    "description": "make fresh fish with technology"
}))]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateCompanyDto {
    pub company_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}


#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "name": "Otter heaven inc",
    "owner_id": "029j9sjfe9032",
    "description": "use tech to grow sweet fish",
    "created_at": "2021-08-01T00:00:00Z",
    "updated_at": "2021-08-01T00:00:00Z"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryCompanyDto{
    pub id : String,
    pub name : String,
    pub owner_id : String,
    pub description : String,
    pub created_at : String,
    pub updated_at : String,
}


#[derive(Serialize,ToSchema)]
#[schema(example = json!({
    "total": 2,
    "companies": [
        {
            "id": "2390whflksjf0993",
            "name": "Otter heaven inc",
            "owner_id": "029j9sjfe9032",
            "description": "use tech to grow sweet fish",
            "created_at": "2021-08-01T00:00:00Z",
            "updated_at": "2021-08-01T00:00:00Z"
        },{
            "id": "230sdfnklseo",
            "name": "Otter Paradise inc",
            "owner_id": "u8futy20lksdnfo0e",
            "description": "better live, better fish",
            "created_at": "2021-08-01T00:00:00Z",
            "updated_at": "2021-08-01T00:00:00Z"
        }
    ]
}))]
#[serde(crate= "rocket::serde")]
pub struct ResListEntryCompanyDto{
    pub total: i32,
    pub companies: Vec<ResEntryCompanyDto>,
}

#[derive(Serialize,ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResCompanyRelateUserDto{
    pub company_id: String,
    pub company_name: String,
    pub users: Vec<ResEntryUserDto>,
    
}


