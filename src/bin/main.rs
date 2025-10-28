use std::net::SocketAddr;

use auth_service::{app, config::APP_CONFIG, utils::tracing::init_standard_tracing};
use auth_service::static_service::{get_blockchain_connection, get_database_connection};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    init_standard_tracing(env!("CARGO_CRATE_NAME"));

    tracing::info!("Starting application...");

    let _db_connection = get_database_connection().await;
    let _init_blockchain_service = get_blockchain_connection().await;

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
