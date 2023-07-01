use async_trait::async_trait;
use crate::data::dto::TodoDto;

#[async_trait]
pub trait ListTodoRepository {
    async fn list(&self, user_id: i32) -> Result<Vec<TodoDto>, sqlx::Error>;
}
