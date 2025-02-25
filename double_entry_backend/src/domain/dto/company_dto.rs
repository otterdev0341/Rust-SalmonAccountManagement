use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::auth_dto::ResEntryUserDto;






#[derive(Deserialize,Serialize,ToSchema, Clone)]
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
    pub name: Option<String>,
    pub description: Option<String>,
}


#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "name": "Otter heaven inc",
    "description": "use tech to grow sweet fish",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryCompanyDto{
    #[schema(value_type = String)]
    pub id : Uuid,
    pub name : String,
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
            "description": "use tech to grow sweet fish",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        },{
            "id": "230sdfnklseo",
            "name": "Otter Paradise inc",
            "description": "better live, better fish",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        }
    ]
}))]
#[serde(crate= "rocket::serde")]
pub struct ResListCompanyDto{
    pub total: u32,
    pub companies: Vec<ResEntryCompanyDto>,
}

#[derive(Serialize,ToSchema)]
#[schema(example = json!({
    "companyName": "Otter heaven inc",
    "totalUser": 2,
    "users": [
        {
            "id": "2390whflksjf0993",
            "username": "kotaro_cute",
            "firstName": "kotaro the otter",
            "lastName": "cute",
            "email": "kotaro@moew.com"
        },{
            "id": "230sdfnklseo",
            "username": "hana_the_princess",
            "firstName": "hana the salmon destroyer",
            "lastName": "cute",
            "email": "hana@moew.com"
        }
    ]
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResCompanyRelateUserDto{
    pub company_name: String,
    pub total_user: u32,
    pub users: Vec<ResEntryUserDto>,
    
}


#[derive(Deserialize,Serialize, ToSchema)]
#[schema(example = json!({
    "companyId": "2390whflksjf0993",
    "userEmail": "kotaro@meow.com"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct AddRemoveUserToCompanyDto{
    pub company_id: String,
    pub user_email: String,
}