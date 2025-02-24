use std::sync::Arc;

use rocket::fairing::AdHoc;
use sea_orm::DatabaseConnection;

use crate::infrastructure::mysql::repositories::impl_auth_repository::ImplAuthRepository;

use super::auth_usecase::AuthUseCase;



pub fn init_usecase_setup(db_connection: Arc<DatabaseConnection>) -> AdHoc {
    AdHoc::on_ignite("Initialize use cases", |rocket| async move {
        

        // Initialize auth usecase
        let auth_repository = ImplAuthRepository{
            db: Arc::clone(&db_connection)
        };
        let user_usecase = Arc::new(AuthUseCase::new(Arc::new(auth_repository)));

        
        
        rocket.manage(Arc::clone(&db_connection))
              .manage(user_usecase)
              
    })
}