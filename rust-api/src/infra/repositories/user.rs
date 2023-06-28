use async_trait::async_trait;
use sqlx::PgPool;

use crate::data::repositories::CreateUserRepository;
use crate::domain::entities::User;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CreateUserRepository for UserRepository {
    async fn create(&self, user: User) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query!(
            r#"
                INSERT INTO "User" (name, email, password)
                VALUES($1, $2, $3)
            "#,
            user.name,
            user.email,
            user.password
        )
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
