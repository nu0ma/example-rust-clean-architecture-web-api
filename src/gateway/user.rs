use anyhow::Ok;
use async_trait::async_trait;

use crate::{domain::user::User, driver::db_driver::DbDriverTrait, port::user::UserPort};

pub struct UserGateway<T: DbDriverTrait> {
    pub db_driver: T,
}

#[async_trait]
impl<T: DbDriverTrait + Send + Sync> UserPort for UserGateway<T> {
    async fn get_user(&self, id: i32) -> anyhow::Result<Vec<User>> {
        let users = self.db_driver.get_user(id).await?;
        let result = users
            .into_iter()
            .map(|user| User {
                id: user.id,
                name: user.name,
            })
            .collect();

        Ok(result)
    }

    async fn create_user(&self, user: User) -> anyhow::Result<()> {
        self.db_driver.create_user(user).await?;
        Ok(())
    }

    async fn update_user(&self, id: i32, name: String) -> anyhow::Result<()> {
        self.db_driver.update_user(id, name).await?;
        Ok(())
    }

    async fn delete_user(&self, id: i32) -> anyhow::Result<()> {
        self.db_driver.delete_user(id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::process::id;

    use crate::{
        domain::user::User,
        driver::db_driver::{MockDbDriverTrait, UserModel},
        port::user::UserPort,
    };
    use anyhow::Ok;
    use mockall::predicate;

    use super::UserGateway;

    #[tokio::test]
    async fn test_get_user() {
        let id = 1;
        let expected = vec![
            User {
                id: 1,
                name: "test1".to_string(),
            },
            User {
                id: 2,
                name: "test2".to_string(),
            },
        ];

        let mut mock_db_driver = MockDbDriverTrait::new();
        mock_db_driver
            .expect_get_user()
            .with(predicate::eq(id))
            .times(1)
            .returning(|_| {
                Ok(vec![
                    UserModel {
                        id: 1,
                        name: "test1".to_string(),
                    },
                    UserModel {
                        id: 2,
                        name: "test2".to_string(),
                    },
                ])
            });

        let user_gateway = UserGateway {
            db_driver: mock_db_driver,
        };

        let actual = user_gateway.get_user(id).await.unwrap();
        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn test_create_user() {
        let user = User {
            id: 1,
            name: "test".to_string(),
        };
        let expected = Ok(()).unwrap();
        let mut mock_db_driver = MockDbDriverTrait::default();

        mock_db_driver
            .expect_create_user()
            .with(predicate::eq(user.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let user_gateway = UserGateway {
            db_driver: mock_db_driver,
        };

        let actual = user_gateway.create_user(user).await.unwrap();
        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn test_update_user() {
        let id = 1;
        let name = "new_name".to_string();

        let expected = Ok(()).unwrap();
        let mut mock_db_driver = MockDbDriverTrait::default();

        mock_db_driver
            .expect_update_user()
            .with(predicate::eq(id), predicate::eq(name.clone()))
            .returning(|_, _| Ok(()));

        let user_gateway = UserGateway {
            db_driver: mock_db_driver,
        };

        let actual = user_gateway.update_user(id, name).await.unwrap();
        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn test_delete_user() {
        let id = 1;

        let mut mock_db_driver = MockDbDriverTrait::default();
        mock_db_driver
            .expect_delete_user()
            .with(predicate::eq(id))
            .times(1)
            .returning(|_| Ok(()));

        let user_gateway = UserGateway {
            db_driver: mock_db_driver,
        };

        let expected = Ok(()).unwrap();
        let actual = user_gateway.delete_user(id).await.unwrap();
        assert_eq!(expected, actual)
    }
}
