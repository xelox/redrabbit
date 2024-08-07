use axum::{routing::post, Router};
mod nodes;

pub fn api_router() -> Router {
    Router::new()
        .route("/tasks/load", post(crate::api::nodes::load))
        .route("/tasks/create", post(crate::api::nodes::create))
        .route("/tasks/delete", post(crate::api::nodes::delete))
        .route("/tasks/expand_collapse", post(crate::api::nodes::expand))
        .route("/tasks/update_completion", post(crate::api::nodes::update_completion))
}
