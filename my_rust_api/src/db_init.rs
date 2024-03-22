pub async fn connect_to_db() -> Router {
    let pool = MySqlPool::connect(api::get_database_url()).
    await
    .expect("Could not connect to the database");

    return Router::new()
    .route("/", get(|| async {"Hello, Rust!"}))
    .route("/create-user", post(api::create_user()))
    .route("/users", get(api::get_users()))
    .route("/item/:id", get(api::show_item()))
    .route("/add-item", post(api::add_item()))
    .route("/delete-user/:user_id", delete(api::delete_user()))
    .route("/remove-user/:user_id", post(api::remove_user()))
    .route("/add-user/:user_id", post(api::add_user()))
    .layer(Extension(pool))
    .layer(middleware::from_fn(logging::logging_middleware()));
}