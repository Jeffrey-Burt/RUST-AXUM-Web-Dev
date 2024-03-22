use chrono::Local;
use axum::{
    body::Body,
    middleware::Next,
    response::Response,
    http::Request
};

pub fn get_current_datetime() -> String {
    return Local::now().format("%m/%d/%Y %H:%M:%S").to_string();
}

pub async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> Response {
    println!("[{}] Received a requst to {}", get_current_datetime(), req.uri());
    next.run(req).await
}