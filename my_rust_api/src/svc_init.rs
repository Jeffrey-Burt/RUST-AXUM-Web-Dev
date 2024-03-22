use axum::Router;

fn get_host_port() -> String {
    return "127.0.0.1:3000".to_string();
}

pub async fn init_svc(app: Router) {
    axum::Server::bind(&get_host_port().parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}