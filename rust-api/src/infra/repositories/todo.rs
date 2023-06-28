use sqlx::PgPool;
use async_trait::async_trait;

use crate::{data::repositories::CreateTodoRepository, domain::entities::Todo};

pub struct TodoRepository {
    pool: PgPool 
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
            None => false
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
