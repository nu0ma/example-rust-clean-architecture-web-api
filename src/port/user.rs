use async_trait::async_trait;
use mockall::automock;

use crate::domain::user::User;

#[automock]
#[async_trait]
pub trait UserPort {
    async fn get_user(&self, id: i32) -> anyhow::Result<Vec<User>>;
    async fn create_user(&self, user: User) -> anyhow::Result<()>;
    async fn update_user(&self, id: i32, name: String) -> anyhow::Result<()>;
    async fn delete_user(&self, id: i32) -> anyhow::Result<()>;
}
