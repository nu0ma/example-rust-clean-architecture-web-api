use async_trait::async_trait;
use mockall::automock;
use sqlx::FromRow;

use crate::{connection_pool::DB_POOL, domain::user::User};

#[derive(FromRow)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DbDriver {}

#[automock]
#[async_trait]
pub trait DbDriverTrait {
    async fn get_user(&self, id: i32) -> anyhow::Result<Vec<UserModel>>;
    async fn create_user(&self, user: User) -> anyhow::Result<()>;
    async fn update_user(&self, id: i32, name: String) -> anyhow::Result<()>;
    async fn delete_user(&self, id: i32) -> anyhow::Result<()>;
}

#[async_trait]
impl DbDriverTrait for DbDriver {
    async fn get_user(&self, id: i32) -> anyhow::Result<Vec<UserModel>> {
        let row = sqlx::query_as::<_, UserModel>("SELECT * FROM USERS where id = $1")
            .bind(id)
            .fetch_all(DB_POOL.get().expect("DB Connection Error"))
            .await?;

        Ok(row)
    }

    async fn create_user(&self, user: User) -> anyhow::Result<()> {
        let sql = "INSERT INTO USERS (id, name) VALUES ($1, $2)";

        sqlx::query(sql)
            .bind(user.id)
            .bind(user.name)
            .execute(DB_POOL.get().expect("DB Connection Error"))
            .await?;
        Ok(())
    }

    async fn update_user(&self, id: i32, name: String) -> anyhow::Result<()> {
        let sql = "UPDATE USERS SET name = $1 where id = $2";

        sqlx::query(sql)
            .bind(name)
            .bind(id)
            .execute(DB_POOL.get().expect("DB Connection Error"))
            .await?;
        Ok(())
    }

    async fn delete_user(&self, id: i32) -> anyhow::Result<()> {
        let sql = "DELETE FROM USERS where id = $1";

        sqlx::query(sql)
            .bind(id)
            .execute(DB_POOL.get().unwrap())
            .await?;
        Ok(())
    }
}
