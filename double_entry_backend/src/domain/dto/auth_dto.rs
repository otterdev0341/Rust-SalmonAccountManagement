
use ::serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;


#[allow(dead_code)]

// Serialize and Deserialize are used to convert the struct into a JSON object and vice versa
// ToSchema is used to generate the OpenAPI schema for the struct

#[derive(Serialize ,Deserialize, ToSchema, Clone)]
#[schema(example = json!({
    "username": "kotaro_cute", 
    "first_name": "kotaro",
    "last_name": "cute",
    "email": "kotaro_work.com",
    "password": "kotaro1235555"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateUserDto {

    
    pub username: String,

    
    pub first_name: String,

    
    pub last_name: String,

    
    pub email: String,

    
    pub password: String,
}





#[derive(Serialize, ToSchema)]
#[schema(example = json!({"id": "2309jfslkjfe90jklfh", "username": "kotaro_cute", "first_name": "kotaro", "last_name": "cute", "email": "kotaro_work.com"}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResEntryUserDto {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}


#[derive(Deserialize, ToSchema)]
#[schema(example = json!({"email": "user@example.com", "password": "securepassword"}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqSignInDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"token": "87AbcDlksdjfiw90w8ljnsLKJFIhwisfhKLDSf"}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResSignInDto {
    pub token: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ClaimsDto {
    pub subject_id: String,
    pub username: String,
    pub exp: u64,
}

// use in faring authentication.rs
// use only internal server, not recive from client nor return to client
pub struct AuthenticatedUser {
    pub id: String,
    pub username: String,  
}


#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example
= json!({
    "firstName": "all new kotaro",
    "lastName": "cute otter",
    "email": "kotaro_note_work.com"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateUserDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}



#[derive(Debug, Deserialize, Serialize,ToSchema)]
#[schema(example = json!({"password":"newpassword"}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdatePasswordDto {
    pub password: String,
}
