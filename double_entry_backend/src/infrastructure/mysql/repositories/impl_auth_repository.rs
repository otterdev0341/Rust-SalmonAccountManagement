use std::sync::Arc;

use rocket::fairing::Info;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use tracing::{info, warn};

use crate::{domain::{dto::auth_dto::{ReqCreateUserDto, ReqSignInDto, ResEntryUserDto}, entities::user, repository::require_implementation::trait_auth::AuthRepoReqImpl}, infrastructure::{argon_hash::hash_util::hash_password, handler::operation_status::auth_error::{CreateUserError, SignInError}}};



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


#[async_trait::async_trait]
impl AuthRepoReqImpl for ImplAuthRepository {
    async fn create_user(&self, user_data: ReqCreateUserDto) -> Result<(), CreateUserError> {
        
        // check username is already exists
        match user::Entity::find().filter(user::Column::Username.eq(user_data.username.clone())).one(&*self.db).await {
            Ok(user) => {
                if let Some(user) = user {
                    return Err(CreateUserError::UsernameAlreadyExists)
                }
            },
            Err(_) => ()
        }
        
        // check is email already exists
        match user::Entity::find().filter(user::Column::Email.eq(user_data.email.clone())).one(&*self.db).await {
            Ok(data) => {
                if let Some(data) = data {
                    return Err(CreateUserError::EmailAlreadyExists)
                }
            },
            Err(_) => ()
        }
        
        // create hash password from user data
        let hash_password = hash_password(&user_data.password).unwrap();
        
        // create use entity to persit to database
        let new_user = user::ActiveModel {
            username: Set(user_data.username),
            email: Set(user_data.email),
            password_hash: Set(hash_password),
            first_name: Set(user_data.first_name),
            last_name: Set(user_data.last_name),
            ..Default::default()
        };
        
        // check insert result
        let persist_result = user::Entity::insert(new_user)
            .exec(&*self.db)
            .await
            .map_err(|err| {
                warn!("{}", err.to_string());
                CreateUserError::InternalServerError
            })?;
   
            
        Ok(())
        

    }

    async fn sign_in(&self, sign_data: ReqSignInDto) -> Result<ResEntryUserDto, SignInError> {
        unimplemented!()
    }
}