use std::sync::Arc;

use rocket::fairing::AdHoc;
use sea_orm::DatabaseConnection;

use crate::infrastructure::mysql::repositories::{impl_auth_repository::ImplAuthRepository, impl_company_repository::ImplCompanyRespository, impl_contact_detail::ImplContactDetailRespository, impl_contact_repository::ImplContactRepository, impl_contact_type_repository::ImplContactTypeRepository, impl_info_repository, impl_project_repository, impl_project_status_repository::ImplProjectStatusRepository};

use super::{auth_usecase::AuthUseCase, company_usecase::CompanyUseCase, contact_detail_usecase::ContactDetailUseCase, contact_type_usecase::ContactTypeUseCase, contact_usecase::ContactUseCase, info_usecase::{self, InfoUseCase}, project_status_usecase::ProjectStatusUseCase, project_usecase::ProjectUseCase};



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
        let contact_type_repository = ImplContactTypeRepository{
            db: Arc::clone(&db_connection)
        };
        let contact_detail_repository = ImplContactDetailRespository{
            db: Arc::clone(&db_connection)
        };
        let project_status_repository = ImplProjectStatusRepository{
            db: Arc::clone(&db_connection)
        };
        let impl_project_repository = impl_project_repository::ImplProjectRepository{
            db: Arc::clone(&db_connection)
        };
        let impl_info_repository = impl_info_repository::ImplInfoRepository{
            db: Arc::clone(&db_connection)
        };

        // Initial Usecase
        let user_usecase = Arc::new(AuthUseCase::new(Arc::new(auth_repository)));
        let company_usecase = Arc::new(CompanyUseCase::new(Arc::new(company_repository)));
        let contact_usecase = Arc::new(ContactUseCase::new(Arc::new(contact_repository)));
        let contact_type_usecase = Arc::new(ContactTypeUseCase::new(Arc::new(contact_type_repository)));
        let contact_detail_usecase = Arc::new(ContactDetailUseCase::new(Arc::new(contact_detail_repository)));
        let project_status_usecase = Arc::new(ProjectStatusUseCase::new(Arc::new(project_status_repository)));
        let project_usecase = Arc::new(ProjectUseCase::new(Arc::new(impl_project_repository)));
        let info_usecase = Arc::new(InfoUseCase::new(Arc::new(impl_info_repository)));

        // Attach to Rocket
        rocket.manage(Arc::clone(&db_connection))
              .manage(user_usecase)
              .manage(company_usecase)
              .manage(contact_usecase)
              .manage(contact_type_usecase)
              .manage(contact_detail_usecase)
              .manage(project_status_usecase)
              .manage(project_usecase)
              .manage(info_usecase)
              
    })
}