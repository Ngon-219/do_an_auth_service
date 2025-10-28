use std::net::SocketAddr;

use auth_service::{app, config::APP_CONFIG, utils::tracing::init_standard_tracing};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    init_standard_tracing(env!("CARGO_CRATE_NAME"));

    tracing::info!("Starting application...");

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
