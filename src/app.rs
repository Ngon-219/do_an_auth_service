use crate::api_docs::ApiDoc;
use crate::config::APP_CONFIG;
use crate::routes;
use crate::routes::health::route::create_route;
use axum::Router;
use http::header;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{ServiceBuilderExt, propagate_header::PropagateHeaderLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn create_app() -> anyhow::Result<Router> {
    let mut router = Router::new()
        .merge(create_route())
        .merge(routes::users::create_route())
        .merge(routes::managers::create_route())
        .merge(routes::students::create_route());

    // Add Swagger UI
    if APP_CONFIG.swagger_enabled {
        let swagger_ui =
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());
        router = router.merge(swagger_ui);
    }

    // Apply middleware
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

    let middleware = ServiceBuilder::new()
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
            "x-request-id",
        )))
        .sensitive_request_headers(sensitive_headers.clone())
        .sensitive_response_headers(sensitive_headers)
        .compression();

    Ok(router.layer(middleware))
}
