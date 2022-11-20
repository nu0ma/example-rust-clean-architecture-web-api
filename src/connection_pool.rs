use anyhow::Ok;
use once_cell::sync::{Lazy, OnceCell};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub static CONNECTION_SETTING: Lazy<Connection> = Lazy::new(|| Connection::new().unwrap());
pub static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn create_pool() -> PgPool {
    let connection_setting = Connection::new().unwrap();

    let db_url = &format!(
        "postgres://{}:{}@{}:{}/{}",
        connection_setting.db_user,
        connection_setting.db_password,
        connection_setting.db_host,
        connection_setting.db_port,
        connection_setting.db_name
    );

    PgPoolOptions::new().connect(db_url).await.unwrap()
}

#[derive(Deserialize, Debug)]
pub struct Connection {
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
}

impl Connection {
    fn new() -> anyhow::Result<Self> {
        Ok(envy::from_env::<Connection>()?)
    }
}
