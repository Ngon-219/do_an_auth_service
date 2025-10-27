use crate::config::APP_CONFIG;
use crate::routes;
use crate::routes::health::route::create_route;
use axum::{Router, middleware};
use http::header;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tower::ServiceBuilder;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_http::{ServiceBuilderExt, cors::CorsLayer, propagate_header::PropagateHeaderLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api_docs::ApiDoc;

pub async fn create_app() -> Router {
    let mut router = Router::new()
        .nest("/health", create_route())
        .nest(
        "/api/v1/",
        Router::new()
            // .merge(routes::health::route::create_route()),
    );

    if APP_CONFIG.swagger_enabled {
        let config = utoipa_swagger_ui::Config::new(["/"]).display_request_duration(true);
        let swagger_ui = SwaggerUi::new("/swagger-ui")
            .url("/", ApiDoc::openapi())
            .config(config);
        router = router.merge(swagger_ui);
    }


    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

    let middleware = ServiceBuilder::new()
        .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
            "x-request-id",
        )))
        .sensitive_request_headers(sensitive_headers.clone())
        // .layer(middleware::from_fn(http_logger))
        .sensitive_response_headers(sensitive_headers)
        .compression();

    Router::new().merge(router).layer(middleware)
}
