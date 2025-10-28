use std::net::SocketAddr;

use auth_service::bootstrap::initialize_admin_user;
use auth_service::static_service::get_database_connection;
use auth_service::{app, config::APP_CONFIG, utils::tracing::init_standard_tracing};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    init_standard_tracing(env!("CARGO_CRATE_NAME"));

    tracing::info!("Starting application...");

    // Initialize database connection
    let db_connection = get_database_connection().await;

    // Initialize default admin user
    tracing::info!("Checking admin user...");
    if let Err(e) = initialize_admin_user(db_connection).await {
        tracing::error!("Failed to initialize admin user: {}", e);
        tracing::warn!("Continuing without admin user initialization...");
    }

    let app = app::create_app().await?;

    let address = format!("0.0.0.0:{}", APP_CONFIG.port);

    tracing::info!("Server listening on {}", &address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Failed to start server");

    Ok(())
}
