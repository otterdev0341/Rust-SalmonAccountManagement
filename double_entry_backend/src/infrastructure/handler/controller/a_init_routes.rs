use rocket::fairing::AdHoc;

use super::{account::account_routes, auth::auth_routes, chart_of_account::chart_of_account_routes, company::company_routes, contact::contact_routes, contact_detail::contact_detail_routes, contact_type::contact_type_routes, info::info_routes, journal_entry::journal_entry_routes, location::location_routes, location_type::location_type_routes, project::project_routes, project_status::project_status_routes, reference_type::reference_type_routes, utility::utility_routes};








pub fn init_controller_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize controller", |rocket| async {
        rocket
            .mount("/v1/check", utility_routes())
            .mount("/v1/auth", auth_routes())
            .mount("/v1/company", company_routes())
            .mount("/v1/account", account_routes())
            .mount("/v1/contact", contact_routes())
            .mount("/v1/contact-detail", contact_detail_routes())
            .mount("/v1/contact-type", contact_type_routes())   
            .mount("/v1/info", info_routes())
            .mount("/v1/project", project_routes())
            .mount("/v1/location", location_routes())
            .mount("/v1/location-type", location_type_routes())
            .mount("/v1/project-status", project_status_routes())
            .mount("/v1/account-balance",account_routes())
            .mount("/v1/chart-of-account", chart_of_account_routes())
            .mount("/v1/reference-type", reference_type_routes())
            .mount("/v1/journal-entry", journal_entry_routes())

            
    })
}
