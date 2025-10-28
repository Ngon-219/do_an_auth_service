use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::HttpBuilder;
use utoipa::openapi::security::SecurityScheme;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::health::route::health_check,
        crate::routes::users::route::create_user,
        crate::routes::users::route::create_users_bulk,
    ),
    components(
        schemas(
            crate::routes::users::dto::CreateUserRequest,
            crate::routes::users::dto::UserResponse,
            crate::routes::users::dto::BulkUserResponse,
            crate::routes::users::dto::BulkUserError,
            crate::entities::sea_orm_active_enums::RoleEnum,
        ),
    ),
    modifiers(&SecurityModifier),
    tags(
        (name = "Users", description = "User management endpoints"),
        (name = "health", description = "Health check endpoints")
    ),
)]
pub struct ApiDoc;

struct SecurityModifier;
impl Modify for SecurityModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        components.add_security_scheme(
            "basic_auth",
            SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Basic).build()),
        );
    }
}
