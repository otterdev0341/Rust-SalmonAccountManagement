use std::sync::Arc;

use rocket::fairing::AdHoc;
use sea_orm::DatabaseConnection;

use crate::{domain::entities::contact, infrastructure::mysql::repositories::{impl_auth_repository::ImplAuthRepository, impl_company_repository::ImplCompanyRespository, impl_contact_repository::ImplContactRepository}};

use super::{auth_usecase::AuthUseCase, company_usecase::CompanyUseCase, contact_usecase::{self, ContactUseCase}};



pub fn init_usecase_setup(db_connection: Arc<DatabaseConnection>) -> AdHoc {
    AdHoc::on_ignite("Initialize use cases", |rocket| async move {
        

        // Initial Repositrory
        let auth_repository = ImplAuthRepository{
            db: Arc::clone(&db_connection)
        };
        let company_repository = ImplCompanyRespository{
            db: Arc::clone(&db_connection)
        };
        let contact_repository = ImplContactRepository{
            db: Arc::clone(&db_connection)
        };

        // Initial Usecase
        let user_usecase = Arc::new(AuthUseCase::new(Arc::new(auth_repository)));
        let company_usecase = Arc::new(CompanyUseCase::new(Arc::new(company_repository)));
        let contact_usecase = Arc::new(ContactUseCase::new(Arc::new(contact_repository)));
        // Attach to Rocket
        
        
        rocket.manage(Arc::clone(&db_connection))
              .manage(user_usecase)
              .manage(company_usecase)
              .manage(contact_usecase)
              
    })
}