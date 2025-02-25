use core::hash;
use std::sync::Arc;

use rocket::fairing::Info;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use thiserror::Error;
use tracing::{info, warn};
use utoipa::openapi::info;
use uuid::Uuid;

use crate::{application::usecase::auth_usecase::AuthUseCaseError, domain::{dto::auth_dto::{ReqCreateUserDto, ReqSignInDto, ResEntryUserDto}, entities::user, repository::require_implementation::trait_auth::AuthRepoReqImpl}, infrastructure::argon_hash::hash_util::{hash_password, verify_password, HashOperationError}};



pub struct ImplAuthRepository {
    pub db: Arc<DatabaseConnection>
}

impl ImplAuthRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplAuthRepository {
            db
        }
    }
}

#[derive(Debug, Error)]
pub enum AuthRepositoryError {
    
    #[error("Database Error: {0}")]
    DatabaseError(#[from] DbErr),
    
    #[error("User not found")]
    UserNotFound,

    #[error("Username already exists")]
    EmailAlreadyExists,

    #[error("Email already exists")]
    UserAlreadyExists,

    #[error("email or password not correct")]
    EmailOrPasswordNotCorrect,
    
    #[error("Email not fount")]
    EmailNotFound,

    #[error("Hash operation failed")]
    HashFailed,

    #[error("Uuid cast error")]
    UuidCastError
}

impl From<AuthRepositoryError> for AuthUseCaseError {
    fn from(error: AuthRepositoryError) -> Self {
        match error {
            AuthRepositoryError::DatabaseError(db) => AuthUseCaseError::InternalError(db.to_string()),
            AuthRepositoryError::UserNotFound => AuthUseCaseError::UserNotFound,
            AuthRepositoryError::EmailAlreadyExists => AuthUseCaseError::EmailAlreadyExists,
            AuthRepositoryError::UserAlreadyExists => AuthUseCaseError::UserAlreadyExists,
            AuthRepositoryError::EmailOrPasswordNotCorrect => AuthUseCaseError::EmailOrPasswordNotCorrect,
            AuthRepositoryError::EmailNotFound => AuthUseCaseError::EmailNotFound,
            AuthRepositoryError::HashFailed => AuthUseCaseError::HashFailed,
            AuthRepositoryError::UuidCastError => AuthUseCaseError::UuidCastError
        }
    }
}

#[async_trait::async_trait]
impl AuthRepoReqImpl for ImplAuthRepository {
    async fn create_user(&self, user_data: ReqCreateUserDto) -> Result<(), AuthRepositoryError> {
        info!(">>>>>>>>>>>> create user in repository");
        info!(">>>>>>>>>>>>> check is  user exists");
        // check username is already exists
        match user::Entity::find().filter(user::Column::Username.eq(user_data.username.clone())).one(&*self.db).await {
            Ok(user) => {
                if let Some(user) = user {
                    return Err(AuthRepositoryError::UserAlreadyExists)
                }
            },
            Err(_) => ()
        }
        
        info!(">>>>>>>>>>>>> check is email already exists");
        // check is email already exists
        match user::Entity::find().filter(user::Column::Email.eq(user_data.email.clone())).one(&*self.db).await {
            Ok(data) => {
                if let Some(data) = data {
                    return Err(AuthRepositoryError::EmailAlreadyExists)
                }
            },
            Err(_) => ()
        }
        
        info!(">>>>>>>>>>>>> create hash password");
        // create hash password from user data
        let hash_password = hash_password(&user_data.password).map_err(|_| HashOperationError::FailToHashPassword);
        if hash_password.is_err() {
            return Err(AuthRepositoryError::EmailOrPasswordNotCorrect)
        }

        info!(">>>>>>>>>>>>> create user entity");
        // create use entity to persit to database
        let new_user = user::ActiveModel {
            username: Set(user_data.username),
            email: Set(user_data.email),
            password_hash: Set(hash_password.unwrap()),
            first_name: Set(user_data.first_name),
            last_name: Set(user_data.last_name),
            ..Default::default()
        };
        
        info!(">>>>>>>>>>>>> insert user entity");
        // check insert result, this is pass
        let persist_result = user::Entity::insert(new_user)
            .exec_without_returning(&*self.db)
            .await;

        info!(">>>>>>>>>>>>> check insert result");
        match persist_result {
            Ok(data) => (),
            Err(_) => {
                warn!("Failed to insert user entity");
                return Err(AuthRepositoryError::DatabaseError(DbErr::RecordNotInserted))
            }
        }
        Ok(())
        

    }





    async fn sign_in(&self, sign_data: ReqSignInDto) -> Result<ResEntryUserDto, AuthRepositoryError> {
        info!(">>>>>>>>>>>> sign in user in repository");
        info!(">>>>>>>>>>>>> check is email exists");
        // check is email exists
        let user = match user::Entity::find().filter(user::Column::Email.eq(sign_data.email.clone())).one(&*self.db).await {
            Ok(data) => {
                if let Some(data) = data {
                    data
                } else {
                    return Err(AuthRepositoryError::EmailNotFound)
                }
            },
            Err(_) => return Err(AuthRepositoryError::DatabaseError(DbErr::RecordNotFound(("User not found").to_string())))
        };
        
        info!(">>>>>>>>>>>>> check password");
        // check password
        let is_password_match = verify_password(&sign_data.password, &user.password_hash);
        match is_password_match {
            Ok(the_bool) => {
                if !the_bool {
                    return Err(AuthRepositoryError::EmailOrPasswordNotCorrect)
                }
            },
            Err(_) => return Err(AuthRepositoryError::HashFailed)
        }
        
        
        info!(">>>>>>>>>>>>> create user to return");
        
        
                let res_user = ResEntryUserDto {
                    id: Uuid::from_slice(&user.id).unwrap(),
                    username: user.username,
                    email: user.email,
                    first_name: user.first_name,
                    last_name: user.last_name
                };
        info!(">>>>>>>>>>>>> return user");
                Ok(res_user)
     


        
    }
}