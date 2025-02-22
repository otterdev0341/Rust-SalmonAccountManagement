use rocket::fairing::AdHoc;

use super::{account::account_routes, auth::auth_routes, company::company_routes, customer::customer_routes, customer_contact::customer_contact_routes, info::info_routes, location::location_routes, location_type::location_type_routes, project::project_routes, utility::utility_routes};








pub fn init_controller_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize controller", |rocket| async {
        rocket
            .mount("/v1/check", utility_routes())
            .mount("/v1/auth", auth_routes())
            .mount("/v1/company", company_routes())
            .mount("/v1/customer", customer_routes())
            .mount("/v1/account", account_routes())
            .mount("/v1/customer-contact", customer_contact_routes())
            .mount("/v1/info", info_routes())
            .mount("/v1/project", project_routes())
            .mount("/v1/location", location_routes())
            .mount("/v1/location-type", location_type_routes())

            
    })
}
