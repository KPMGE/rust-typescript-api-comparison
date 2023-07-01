use async_trait::async_trait;
use crate::data::dto::UpdateTodoDto;

#[async_trait]
pub trait UpdateTodoRepository {
    async fn update(&self, new_todo: &UpdateTodoDto, todo_id: i32) -> Result<(), sqlx::Error>;
}
