use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    data::{
        repositories::{CreateTodoRepository, ListTodoRepository},
        services::TodoDto,
    },
    domain::entities::Todo,
};

pub struct TodoRepository {
    pool: PgPool,
}

impl TodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CreateTodoRepository for TodoRepository {
    async fn create(&self, todo: Todo, user_id: i32) -> Result<(), sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        let completed = match todo.completed {
            Some(val) => val,
            None => false,
        };

        sqlx::query!(
            r#"
                INSERT INTO "Todo" (title, description, completed, "userId") 
                VALUES ($1, $2, $3, $4)
            "#,
            todo.title,
            todo.description,
            completed,
            user_id
        )
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl ListTodoRepository for TodoRepository {
    async fn list(&self, user_id: i32) -> Result<Vec<TodoDto>, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        let todos = sqlx::query_as!(
            TodoDto,
            r#"
                SELECT id, title, description, completed
                FROM "Todo"
                WHERE "userId" = $1
            "#,
            user_id
        )
        .fetch_all(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(todos)
    }
}
