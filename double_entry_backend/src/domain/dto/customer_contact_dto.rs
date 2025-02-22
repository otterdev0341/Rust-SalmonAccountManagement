use serde::{Deserialize, Serialize};
use utoipa::ToSchema;




#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "customerId": "2390whflksjf0993",
    "mobilePhone1": "08123456789",
    "mobilePhone2": "08123456789",
    "mobilePhone3": "08123456789",
    "email": "jojo@work.com",
    "address": "bank road, lagos"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateCustomerContactDto {
    pub customer_id: String,
    pub mobile_phone_1: Option<String>,
    pub mobile_phone_2: Option<String>,
    pub mobile_phone_3: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>
}



#[derive(Serialize, ToSchema)]
#[schema(example = json!({
    "id": "2390whflksjf0993",
    "customerId": "2390whflksjf0993",
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
pub struct ResEntryCustomerContactDto {
    pub id: String,
    pub customer_id: String,
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
    "customerContacts": [
        {
            "id": "2390whflksjf0993",
            "customerId": "2390whflksjf0993",
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
            "customerId": "2390whflksjf0993",
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
pub struct ResListCustomerContactDto {
    pub total: i32,
    pub customer_contacts: Vec<ResEntryCustomerContactDto>
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
pub struct ReqUpdateCustomerContactDto {
    pub mobile_phone_1: Option<String>,
    pub mobile_phone_2: Option<String>,
    pub mobile_phone_3: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>
}