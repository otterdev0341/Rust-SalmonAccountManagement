use rocket::serde;
use ::serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[allow(dead_code)]

// Serialize and Deserialize are used to convert the struct into a JSON object and vice versa
// ToSchema is used to generate the OpenAPI schema for the struct

#[derive(Serialize ,Deserialize, ToSchema)]
#[schema(example = json!({
    "username": "kotaro_cute", 
    "first_name": "kotaro",
    "last_name": "cute",
    "email": "kotaro_work.com",
    "password": "kotaro1235555"
}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
#[schema(example = json!({"email": "user@example.com", "password": "securepassword"}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"token": "87AbcDlksdjfiw90w8ljnsLKJFIhwisfhKLDSf"}))]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn {
    pub token: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub subject_id: Uuid,
    pub username: String,
    pub exp: u64,
}

// use in faring authentication.rs
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub username: String,  
}