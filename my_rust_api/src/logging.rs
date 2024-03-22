use chrono::Local;
use axum::{
    body::Body,
    middleware::Next,
    response::Response,
    http::Request
};

fn get_current_datetime() -> String {
    return Local::now().format("%m/%d/%Y %H:%M:%S").to_string();
}

pub async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> Response {
    println!("[{}] Received {} to {}", get_current_datetime(), req.method(), req.uri());
    next.run(req).await
}