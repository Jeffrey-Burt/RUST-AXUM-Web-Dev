use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::{Path, Query, Json},
    routing::{get, post},
    Router,
};
use serde::{
    Serialize, 
    Deserialize};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String
}

#[derive(Deserialize)]
struct Item {
    title: String,
}
#[derive(Deserialize)]
struct Page {
    number: u32,
}

async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}

async fn add_item(Json(item): Json<Item>) -> String {
    format!("Added item: {}", item.title)
}

async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Jeffrey".to_string(),
            email: "jeffrey@email.com".to_string(),
        },
        User {
            id: 2,
            name: "Zach".to_string(),
            email: "zach@zach.com".to_string(),
        },
    ];
    Json(users)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {"Hello, Rust!"}))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users))
        .route("/item/:id", get(show_item))
        .route("/add-item", post(add_item));

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}