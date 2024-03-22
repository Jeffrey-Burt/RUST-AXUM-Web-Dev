use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    extract::{Path, Query, Json, Extension},
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

#[derive(Serialize)]
pub struct User {
    id: u64,
    name: String,
    email: String
}

#[derive(Serialize)]
pub struct User2 {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
pub struct Item {
    title: String,
}
#[derive(Deserialize)]
pub struct Page {
    number: u32,
}

pub async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number)
}

pub async fn add_item(Json(item): Json<Item>) -> String {
    format!("Added item: {}", item.title)
}

pub async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

pub async fn list_users() -> Json<Vec<User>> {
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

pub async fn delete_user(Path(user_id): Path<u64>) -> Result<Json<User2>, impl IntoResponse> {
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

pub async fn perform_delete_user(user_id: u64) -> Result<(), String> {
    if user_id == 1 {
        Err("User cannot be deleted.".to_string())
    } else {
        Ok(())
    }
}


pub async fn get_users(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
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
pub async fn remove_user(Extension(pool): Extension<MySqlPool>, Path(user_id): Path<u64>) -> impl IntoResponse {
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

pub async fn add_user(Extension(pool): Extension<MySqlPool>, Path(user_id): Path<u64>) -> impl IntoResponse {
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