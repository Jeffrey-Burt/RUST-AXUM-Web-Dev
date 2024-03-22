use axum::{
    Router,
    routing::get,
    extract::Extension,
    middleware::self,
};
use sqlx::MySqlPool;
use crate::logging::logging_middleware;

fn get_database_url() -> String {
    return "mysql://root:Mysqlpassword123@localhost:3306/world".to_string();
}

pub async fn connect_to_db() -> Router {
    let pool = MySqlPool::connect(&get_database_url()).
    await
    .expect("Could not connect to the database");

    return Router::new()
    .route("/", get(|| async {"Hello, Rust!"}))
    .layer(Extension(pool))
    .layer(middleware::from_fn(logging_middleware))
    /*.route("/create-user", post(api::create_user()))
    .route("/users", get(api::get_users()))
    .route("/item/:id", get(api::show_item()))
    .route("/add-item", post(api::add_item()))
    .route("/delete-user/:user_id", delete(api::delete_user()))
    .route("/remove-user/:user_id", post(api::remove_user()))
    .route("/add-user/:user_id", post(api::add_user()))*/
}