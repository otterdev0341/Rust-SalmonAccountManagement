use rocket::serde;
use ::serde::{Deserialize, Serialize};
use utoipa::ToSchema;


// Serialize and Deserialize are used to convert the struct into a JSON object and vice versa
// ToSchema is used to generate the OpenAPI schema for the struct

#[derive(Serialize ,Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateUser {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}