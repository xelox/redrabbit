use axum::{routing::{get, post}, Router};
mod nodes;

pub fn api_router() -> Router {
    Router::new()
        .route("/tasks/load", post(crate::api::nodes::load))
        .route("/tasks/create", post(crate::api::nodes::create))
        .route("/tasks/delete", post(crate::api::nodes::delete))
}
