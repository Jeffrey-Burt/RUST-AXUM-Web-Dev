use axum::{
    body::Body,
    http::{StatusCode, Request},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    extract::{Path, Query, Json, Extension},
    routing::{get, post, delete},
    Router,
    Server,
};
use serde::{
    Serialize, 
    Deserialize
};
use serde_json::json;
use sqlx::{
    MySqlPool,
    Row,
};
use chrono::Local;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String
}

#[derive(Serialize)]
struct User2 {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
struct Item {
    title: String,
}
#[derive(Deserialize)]
struct Page {
    number: u32,
}

fn get_current_datetime() -> String {
    return Local::now().format("%m/%d/%Y %H:%M:%S").to_string();
}

async fn logging_middleware(req: Request<Body>, next: Next<Body>) -> Response {
    println!("[{}] Received a requst to {}", get_current_datetime(), req.uri());
    next.run(req).await
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

async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<User2>, impl IntoResponse> {
    match perform_delete_user(user_id).await {
        Ok(_) => Ok(Json(User2 {
            id: user_id,
            name: "Deleted User".into(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        )),
    }
}

async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    if user_id == 1 {
        Err("User cannot be deleted.".to_string())
    } else {
        Ok(())
    }
}


async fn get_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

// {:?} in formatting to print a vector
async fn remove_user(Extension(pool): Extension<MySqlPool>, Path(user_id): Path<u64>) -> impl IntoResponse {
    let user_to_remove = format!("DELETE FROM users WHERE id={}", user_id);
    let remove_user = match sqlx::query(&user_to_remove)
        .fetch_all(&pool)
        .await
    {
        Ok(remove_user) => {
            if remove_user.is_empty() {
                println!("User successfully removed or user did not exist")
            }
            remove_user
        },
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                println!("Failed to remove user {} from table", user_id),
            )
                .into_response()
        }
    };

    println!("{:?}", remove_user);

    let rows = match sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    println!("Users:\n{:?}", &users);

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

async fn add_user(Extension(pool): Extension<MySqlPool>, Path(user_id): Path<u64>) -> impl IntoResponse {
    let user_to_add = format!("INSERT INTO users (id, name, email)
                                  VALUES ({}, 'Justin Flinch-Fletcher', 'justin.ff@email.com')", user_id);
    let add_user = match sqlx::query(&user_to_add)
        .fetch_all(&pool)
        .await
    {
        Ok(add_user) => {
            if add_user.is_empty() {
                println!("User successfully removed or user did not exist")
            }
            add_user
        },
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                println!("Failed to remove user {} from table", user_id),
            )
                .into_response()
        }
    };

    println!("{:?}", add_user);

    let rows = match sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            )
                .into_response()
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {
            json!({
                "id": row.try_get::<i32, _>("id").unwrap_or_default(),
                "name": row.try_get::<String, _>("name").unwrap_or_default(),
                "email": row.try_get::<String, _>("email").unwrap_or_default(),
            })
        })
        .collect();

    println!("Users:\n{:?}", &users);

    (axum::http::StatusCode::OK, Json(users)).into_response()
}

/**
async fn MySqlPool::connect(database_url: &str) -> Result<Pool<DB>, Error> {
    database_url = "mysql://root:root@localhost:3306/world"
}**/


#[tokio::main]
async fn main() {
    let database_url = "mysql://root:Mysqlpassword123@localhost:3306/world";
    let pool = MySqlPool::connect(&database_url).
        await
        .expect("Could not connect to the database");
    let app = Router::new()
        .route("/", get(|| async {"Hello, Rust!"}))
        .route("/create-user", post(create_user))
        .route("/users", get(get_users))
        .route("/item/:id", get(show_item))
        .route("/add-item", post(add_item))
        .route("/delete-user/:user_id", delete(delete_user))
        .route("/remove-user/:user_id", post(remove_user))
        .route("/add-user/:user_id", post(add_user))
        .layer(Extension(pool))
        .layer(middleware::from_fn(logging_middleware));

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}