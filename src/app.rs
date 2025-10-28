use crate::config::APP_CONFIG;
use crate::routes;
use crate::routes::health::route::create_route;
use crate::blockchain::BlockchainService;
use crate::state::AppState;
use axum::Router;
use http::header;
use sea_orm::Database;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{ServiceBuilderExt, propagate_header::PropagateHeaderLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api_docs::ApiDoc;

pub async fn create_app() -> anyhow::Result<Router> {
    // Initialize database connection
    let db = Database::connect(&APP_CONFIG.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;
    
    // Initialize blockchain service
    let blockchain = BlockchainService::new()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize blockchain service: {}", e))?;
    
    // Create application state
    let state = AppState::new(db, blockchain);
    
    let mut router = Router::new()
        .merge(create_route())
        .nest(
            "/api/v1",
            routes::users::create_route()
        )
        .with_state(state);

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

    Ok(Router::new().merge(router).layer(middleware))
}
