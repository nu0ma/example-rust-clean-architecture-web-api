use anyhow::Ok;

use crate::{domain::user::User, port::user::UserPort};

pub async fn get_user(user_port: impl UserPort, id: i32) -> anyhow::Result<Vec<User>> {
    let user = user_port.get_user(id).await?;
    Ok(user)
}

pub async fn create_user(user_port: impl UserPort, user: User) -> anyhow::Result<()> {
    Ok(user_port.create_user(user).await?)
}

#[cfg(test)]
mod test {
    use crate::{domain::user::User, port::user::MockUserPort, usecase};
    use anyhow::Ok;
    use mockall::predicate;

    #[tokio::test]
    async fn test_get_user() {
        let expected: Vec<User> = vec![];
        let user_id = 1;

        let mut user_port = MockUserPort::new();

        user_port
            .expect_get_user()
            .with(predicate::eq(user_id))
            .times(1)
            .returning(|_| Ok(vec![]));

        let actual = usecase::user::get_user(user_port, user_id).await.unwrap();
        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn test_create_user() {
        let expected = Ok(()).unwrap();

        let user = User {
            id: 1,
            name: "test".to_string(),
        };

        let mut user_port = MockUserPort::default();

        user_port
            .expect_create_user()
            .times(1)
            .with(predicate::eq(user.clone()))
            .returning(|_| Ok(()));

        let actual = usecase::user::create_user(user_port, user).await.unwrap();
        assert_eq!(expected, actual)
    }
}
