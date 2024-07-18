use axum::{routing::{get, post}, Router};
mod nodes;

pub fn api_router() -> Router {
    Router::new()
        .route("/tasks/load", post(crate::api::nodes::load))
}
