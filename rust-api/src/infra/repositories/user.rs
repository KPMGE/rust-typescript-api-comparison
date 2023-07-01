use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::User;
use crate::data::{ 
    repositories::{
        CreateUserRepository, DeleteUserRepository, ListUserRepository, UpdateUserRepository,
    },
    dto::{UpdateUserDto, UserDto},
};

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
    async fn create(&self, user: &User) -> Result<(), sqlx::Error> {
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

#[async_trait]
impl ListUserRepository for UserRepository {
    async fn list(&self) -> Result<Vec<UserDto>, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        let users = sqlx::query_as!(
            UserDto,
            r#"
                SELECT id, name, email 
                FROM "User"
            "#,
        )
        .fetch_all(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(users)
    }
}

#[async_trait]
impl UpdateUserRepository for UserRepository {
    async fn update(&self, user_id: i32, user: &UpdateUserDto) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query!(
            r#"
                UPDATE "User"
                SET name = $1, email = $2
                WHERE id = $3
            "#,
            user.name,
            user.email,
            user_id
        )
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl DeleteUserRepository for UserRepository {
    async fn delete(&self, user_id: i32) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        sqlx::query!(
            r#"
                DELETE FROM "User"
                WHERE id = $1
            "#,
            user_id
        )
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
