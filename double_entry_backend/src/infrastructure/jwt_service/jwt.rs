use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use thiserror::Error;
use uuid::Uuid;
use crate::{config::jwt_config::JwtConfig, domain::dto::auth_dto::{ClaimsDto, ResSignInDto}};


#[derive(Error, Debug)]
pub enum JwtError {
    #[error("SystemTimeError")]
    SystemTimeError,
    #[error("Jwt Env Load Error")]
    EnvError,
    #[error("Jwt Encode Error")]
    JwtEncodeError,
    #[error("Jwt Decode Error")]
    DecodeError
}


pub fn generate_jwt(user_id: Uuid, username: &str) -> Result<ResSignInDto, JwtError> {
    // load env and handle error
    let jwt_config = JwtConfig::default();
    if jwt_config.secret_key.is_empty() || jwt_config.expiration == 0 {
        return Err(JwtError::EnvError);
    }
    // inject the time and handle time error
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| JwtError::SystemTimeError)?;
    let expiration_time = current_time.as_secs() + jwt_config.expiration as u64;

    // create the claim
    let claims = ClaimsDto{
        subject_id: user_id,
        username: username.to_string(),
        exp: expiration_time,
    };

    // Create the encoding key useing the jwt secret
    let encoding_key = EncodingKey::from_secret(jwt_config.secret_key.as_ref());

    // Define Algorithm of Encoding Header
    let header = Header {
        alg: Algorithm::HS512,
        ..Default::default()
    };

    // Encode the JWT and handle the error
    let token = encode(&header, &claims, &encoding_key);
    
    match token {
        Ok(token) => Ok(ResSignInDto {
            token
        }),
        Err(_) => Err(JwtError::JwtEncodeError)
    }

}

pub fn decode_jwt(token: &str) -> Result<ClaimsDto, JwtError> {
    // load env and handing error
    let jwt_config = JwtConfig::default();
    if jwt_config.secret_key.is_empty() {
        return Err(JwtError::EnvError);
    }

    // Create the encoding key using the jwt secret
    let decoding_key = DecodingKey::from_secret(jwt_config.secret_key.as_ref());

    let decoded = decode::<ClaimsDto>(
        token,
        &decoding_key,
        &Validation::new(Algorithm::HS512)
    );
    match decoded {
        Ok(token) => Ok(token.claims),
        Err(_) => Err(JwtError::DecodeError)
    }

}