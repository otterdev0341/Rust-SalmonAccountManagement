use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    security(
        ("bearer_auth" = ["bearer"])
    ),
    paths (
        crate::infrastructure::handler::controller::location_type::view_location_type,
        crate::infrastructure::handler::controller::location_type::view_location_types,
        crate::infrastructure::handler::controller::location_type::edit_location_type,
        crate::infrastructure::handler::controller::location_type::delete_location_type,
        crate::infrastructure::handler::controller::location_type::create_location_type
    ),
    components(
        schemas(
            crate::domain::dto::location_type_dto::ReqCreateLocationTypeDto,
            crate::domain::dto::location_type_dto::ResEntryLocationTypeDto,
            crate::domain::dto::location_type_dto::ResListLocationTypeDto,
            crate::domain::dto::location_type_dto::ReqUpdateLocationTypeDto
        )
    )
)]
pub struct LocationTypeApi;