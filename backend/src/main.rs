use tokio_util::io::ReaderStream;
use axum::{body::Body, http::{header, HeaderMap, StatusCode}, response::IntoResponse, routing::get, Router};
use tower_http::{cors::{self, CorsLayer}, services::ServeDir};
pub mod database;


#[tokio::main]
async fn main() {

    dbg!(database::nodes::Node::load(None));

    let app = Router::new()
        .route("/app", get(serve_app))
        .layer(CorsLayer::new().allow_origin(cors::Any))
        .nest_service("/assets", ServeDir::new("../frontend/dist/assets"))
        .nest_service("/", ServeDir::new("../../frontend/dist"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn serve_app() -> impl IntoResponse {
    let file = match tokio::fs::File::open("../frontend/dist/index.html").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
    headers.insert(header::CONTENT_DISPOSITION, "inline; filename=\"index.html\"".parse().unwrap());

    Ok((headers, body))
}
