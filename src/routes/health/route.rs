use axum::{Router, response::IntoResponse, routing::get};
use crate::state::AppState;

pub fn create_route() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

#[utoipa::path(
    get,
    tag = "health",
    path = "/health",
    responses(
      (status = 200, description = "Health check successful"),
      (status = 500, description = "Health check failed")
    )
)]
async fn health_check() -> impl IntoResponse {
    "OK"
}