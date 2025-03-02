use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::contact_dto::ResListContactDto;




#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "contactId": "2390whflksjf0993",
    "mobilePhone1": "08123456789",
    "mobilePhone2": "08123456789",
    "mobilePhone3": "08123456789",
    "email": "jojo@work.com",
    "address": "bank road, lagos"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateContactDetailDto {
    pub mobile_phone_1: Option<String>,
    pub mobile_phone_2: Option<String>,
    pub mobile_phone_3: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>
}



#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "userId": "2390whflksjf0993",
    "contactId": "2390whflksjf0993",
    "mobilePhone1": "08123456789",
    "mobilePhone2": "08123456789",
    "mobilePhone3": "08123456789",
    "email": "love_to_work@gmail.com",
    "address": "bank road, lagos",
    "createdAt": "2021-08-01T00:00:00Z",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryContactDetailDto {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub contact_id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub mobile_phone_1: String,
    pub mobile_phone_2: String,
    pub mobile_phone_3: String,
    pub email: String,
    pub address: String,
    pub created_at: String,
    pub updated_at: String
}






#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "total": 2,
    "contactDetails": [
        {
            "id": "2390whflksjf0993",
            "contactId": "2390whflksjf0993",
            "mobilePhone1": "08123456789",
            "mobilePhone2": "08123456789",
            "mobilePhone3": "08123456789",
            "email": "",
            "address": "bank road, lagos",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        },
        {
            "id": "2390whflksjf0993",
            "contactId": "2390whflksjf0993",
            "mobilePhone1": "08123456789",
            "mobilePhone2": "08123456789",
            "mobilePhone3": "08123456789",
            "email": "",
            "address": "bank road, lagos",
            "createdAt": "2021-08-01T00:00:00Z",
            "updatedAt": "2021-08-01T00:00:00Z"
        }
    ]
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResListContactDetailDto {
    pub total: u32,
    pub contact_details: Vec<ResEntryContactDetailDto>
}



#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "mobilePhone1": "08123456789",
    "mobilePhone2": "08123456789",
    "mobilePhone3": "08123456789",
    "email": "",
    "address": "bank road, lagos"
}))]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct ReqUpdateContactDetailDto {
    pub mobile_phone_1: Option<String>,
    pub mobile_phone_2: Option<String>,
    pub mobile_phone_3: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>
}


#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "mobilePhone1": "08123456789",
    "mobilePhone2": "08123456783",
    "mobilePhone3": "08123456782",
    "email": "kriky@aloha.com",
    "address": "bank road, lagos",
    "updatedAt": "2021-08-01T00:00:00Z"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResUpdateContactDetailDto {
    #[schema(value_type = String)]
    pub id : Uuid,
    pub mobile_phone_1 : String,
    pub mobile_phone_2 : String,
    pub mobile_phone_3 : String,
    pub email : String,
    pub address : String,
    pub updated_at : String,
}