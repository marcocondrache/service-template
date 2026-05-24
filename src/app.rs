mod health;

use axum::{Router, http::StatusCode, routing::any};

pub fn router() -> Router {
    Router::new()
        .route("/", any(async || StatusCode::NO_CONTENT))
        .merge(health::router())
}
