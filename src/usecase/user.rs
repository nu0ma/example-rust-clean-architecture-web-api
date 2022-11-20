use anyhow::Ok;

use crate::{domain::user::User, port::user::UserPort};

pub async fn get_user(user_port: impl UserPort, id: i32) -> anyhow::Result<Vec<User>> {
    let user = user_port.get_user(id).await?;
    Ok(user)
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
}
