use axum::{
    Router,
    extract::Extension,
    middleware::self,
    routing::{get, post, delete},
};
use sqlx::MySqlPool;
use crate::logging::logging_middleware;
use crate::api::*;

fn get_database_url() -> String {
    return "mysql://root:Mysqlpassword123@localhost:3306/world".to_string();
}

pub async fn connect_to_db() -> Router {
    let pool = MySqlPool::connect(&get_database_url()).
    await
    .expect("Could not connect to the database");

    return Router::new()
    .route("/", get(|| async {"Hello, Rust!"}))
    .route("/create-user", post(create_user))
    .route("/users", get(get_users))
    .route("/item/:id", get(show_item))
    .route("/add-item", post(add_item))
    .route("/delete-user/:user_id", delete(delete_user))
    .route("/remove-user/:user_id", post(remove_user))
    .route("/add-user/:user_id", post(add_user))
    .layer(Extension(pool))
    .layer(middleware::from_fn(logging_middleware))
}