use rocket::fairing::AdHoc;

use super::{auth::auth_routes, company::company_routes, utility::utility_routes};








pub fn init_controller_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize controller", |rocket| async {
        rocket
            .mount("/health/v1", utility_routes())
            .mount("/auth/v1", auth_routes())
            .mount("/company/v1", company_routes())

            
    })
}
