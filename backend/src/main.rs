use axum::{ http::{header, HeaderMap}, response::IntoResponse, routing::get, Router};
use tower_http::{cors::{self, CorsLayer}, services::ServeDir};
pub mod database;
pub mod api;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/app", get(serve_app))
        .nest("/api", api::api_router())
        .layer(CorsLayer::new().allow_origin(cors::Any))
        .nest_service("/assets", ServeDir::new("../frontend/dist/assets"))
        .nest_service("/", ServeDir::new("../../frontend/dist"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn serve_app() -> impl IntoResponse {
    let html = include_str!("../../frontend/dist/index.html");

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
    headers.insert(header::CONTENT_DISPOSITION, "inline; filename=\"index.html\"".parse().unwrap());

    <(HeaderMap, &str) as IntoResponse>::into_response((headers, html))
}
