use axum::{Router, http::StatusCode, routing::get};

pub fn router() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> StatusCode {
    StatusCode::NO_CONTENT
}
