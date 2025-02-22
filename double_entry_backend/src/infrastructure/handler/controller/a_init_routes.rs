use rocket::fairing::AdHoc;

use super::{auth::auth_routes, company::company_routes, customer::customer_routes, customer_contact::customer_contact_routes, utility::utility_routes};








pub fn init_controller_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize controller", |rocket| async {
        rocket
            .mount("/health/v1", utility_routes())
            .mount("/auth/v1", auth_routes())
            .mount("/company/v1", company_routes())
            .mount("/customer/v1", customer_routes())
            .mount("/customer/contact/v1", customer_contact_routes())

            
    })
}
