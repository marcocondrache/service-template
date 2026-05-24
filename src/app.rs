mod health;

use axum::{Router, http::StatusCode, routing::any};

pub fn router() -> Router {
    Router::new().route("/", any(index)).merge(health::router())
}

async fn index() -> StatusCode {
    StatusCode::NO_CONTENT
}
