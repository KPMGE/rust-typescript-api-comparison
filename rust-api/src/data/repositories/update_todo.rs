use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateTodoDto {
    pub title: String,
    pub description: String,
}

#[async_trait]
pub trait UpdateTodoRepository {
    async fn update(&self, new_todo: UpdateTodoDto, todo_id: i32) -> Result<(), sqlx::Error>;
}
