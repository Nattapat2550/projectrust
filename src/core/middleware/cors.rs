use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any) // หรือระบุเจาะจงเช่น "https://myapp.com".parse::<HeaderValue>().unwrap()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_headers(Any)
}