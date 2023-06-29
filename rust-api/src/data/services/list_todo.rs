use crate::data::repositories::ListTodoRepository;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct TodoDto {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

pub async fn list_todo_service(
    repo: Arc<impl ListTodoRepository>,
    user_id: i32,
) -> Result<Vec<TodoDto>, sqlx::Error> {
    repo.list(user_id).await
}
