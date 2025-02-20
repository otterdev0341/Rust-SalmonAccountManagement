use rocket::serde;
use ::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(dead_code)]

// Serialize and Deserialize are used to convert the struct into a JSON object and vice versa
// ToSchema is used to generate the OpenAPI schema for the struct

#[derive(Serialize ,Deserialize, ToSchema)]
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
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn {
    pub token: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub subject_id: i32,
    pub username: String,
    pub exp: u64,
}