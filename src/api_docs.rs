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
        crate::routes::managers::route::add_manager,
        crate::routes::managers::route::remove_manager,
        crate::routes::managers::route::get_all_managers,
        crate::routes::managers::route::check_manager,
        crate::routes::students::route::get_student_by_id,
        crate::routes::students::route::get_student_id_by_address,
        crate::routes::students::route::get_student_id_by_code,
        crate::routes::students::route::deactivate_student,
        crate::routes::students::route::activate_student,
        crate::routes::students::route::check_student_active,
        crate::routes::students::route::get_system_info,
    ),
    components(
        schemas(
            crate::routes::users::dto::CreateUserRequest,
            crate::routes::users::dto::UserResponse,
            crate::routes::users::dto::BulkUserResponse,
            crate::routes::users::dto::BulkUserError,
            crate::routes::managers::dto::AddManagerRequest,
            crate::routes::managers::dto::RemoveManagerRequest,
            crate::routes::managers::dto::ManagerResponse,
            crate::routes::managers::dto::ManagerListResponse,
            crate::routes::managers::dto::CheckManagerRequest,
            crate::routes::students::dto::StudentIdRequest,
            crate::routes::students::dto::StudentAddressRequest,
            crate::routes::students::dto::StudentCodeRequest,
            crate::routes::students::dto::StudentInfoResponse,
            crate::routes::students::dto::StudentStatusResponse,
            crate::routes::students::dto::StudentIdResponse,
            crate::routes::students::dto::SystemInfoResponse,
            crate::entities::sea_orm_active_enums::RoleEnum,
        ),
    ),
    modifiers(&SecurityModifier),
    tags(
        (name = "Users", description = "User management endpoints"),
        (name = "Managers", description = "Manager management endpoints"),
        (name = "Students", description = "Student information endpoints"),
        (name = "System", description = "System information endpoints"),
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
