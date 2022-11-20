use actix_web::{App, HttpServer};
use example_rust_clean_architecture_web_api::{connection_pool, rest::user::get_user};
use tracing::Level;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let db_pool = connection_pool::create_pool().await;
    connection_pool::DB_POOL
        .set(db_pool)
        .expect("DB Connection Error");

    HttpServer::new(|| App::new().service(get_user))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
