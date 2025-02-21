use utoipa::{openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme}, Modify};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}




// impl Modify for SecurityAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
//         components.add_security_scheme(
//             "api_key",
//             SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
//         )
//     }
// }

// fn add_security_scheme(openapi: &mut utoipa::openapi::OpenApi) {
//     openapi.components = Some(
//         utoipa::openapi::ComponentsBuilder::new()
//             .security_scheme(
//                 "bearer_auth",
//                 SecurityScheme::Http(
//                     utoipa::openapi::security::HttpBuilder::new()
//                         .scheme(HttpAuthScheme::Bearer)
//                         .bearer_format("JWT")
//                         .build(),
//                 ),
//             )
//             .build(),
//     );
// }
