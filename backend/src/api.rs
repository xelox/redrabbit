use axum::{routing::get, Router};
mod nodes;

pub fn api_router() -> Router {
    Router::new()
        .route("/tasks/load", get(crate::api::nodes::load))
}
