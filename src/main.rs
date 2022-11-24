use actix_web::{web, App, HttpServer};
use example_rust_clean_architecture_web_api::{
    connection_pool,
    driver::{self},
    rest::user::{create_user, delete_user, get_user, update_user, AppState},
};
use tracing::Level;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let db_pool = connection_pool::create_pool().await;
    connection_pool::DB_POOL
        .set(db_pool)
        .expect("DB Connection Error");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                db_driver: driver::db_driver::DbDriver::default(),
            }))
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
