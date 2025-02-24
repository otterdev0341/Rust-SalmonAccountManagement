use std::error::Error;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use password_hash::{rand_core::OsRng, SaltString, PasswordHasher};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum HashOperationError {
    #[error("Cannot hash password")]
    FailToHashPassword,
    #[error("Cannot verify password")]
    FailOnVerifyPhase,
}




pub fn hash_password(password: &str) -> Result<String, HashOperationError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt);
    
    match password_hash {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err(HashOperationError::FailToHashPassword),
    }
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, HashOperationError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hashed);
    
    match parsed_hash {
        Ok(hash) => {
            let is_valid = argon2.verify_password(password.as_bytes(), &hash).is_ok();
            Ok(is_valid)
        }
        Err(_) => Err(HashOperationError::FailOnVerifyPhase),
    } 
}