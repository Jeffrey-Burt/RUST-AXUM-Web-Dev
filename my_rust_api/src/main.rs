mod svc_init;
mod db_init;
pub mod logging;
pub mod api;

#[tokio::main]
async fn main() {
    println!("Running on http://localhost:3000");
    let app = db_init::connect_to_db();
    svc_init::init_svc(app.await).await;
}