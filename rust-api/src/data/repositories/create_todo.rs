use async_trait::async_trait;

use crate::domain::entities::Todo;

#[async_trait]
pub trait CreateTodoRepository {
    async fn create(&self, todo: &Todo, user_id: i32) -> Result<(), sqlx::Error>;
}
