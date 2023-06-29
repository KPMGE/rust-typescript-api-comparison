use async_trait::async_trait;

#[async_trait]
pub trait DeleteTodoRepository {
    async fn delete(&self, todo_id: i32) -> Result<(), sqlx::Error>;
}
