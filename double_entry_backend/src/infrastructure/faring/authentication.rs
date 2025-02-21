use rocket::{http::Status, outcome::Outcome, request::{self, FromRequest}, Request};
use crate::{domain::dto::auth_dto::AuthenticatedUser, infrastructure::jwt_service::jwt::decode_jwt};



#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(auth_header) = req.headers().get_one("Authorization") {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                

                let data = decode_jwt(token);

                let claims = match data {
                    Ok(p) => p,
                    Err(_) => {
                        return Outcome::Error((Status::Unauthorized, "Invalid token".to_string()))
                    }
                };            

                return Outcome::Success(AuthenticatedUser { id: claims.subject_id, username: claims.username });
            }
        }
        Outcome::Error((Status::Unauthorized, "Authorization header missing or malformed".to_string()))
    }
}