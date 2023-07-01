use crate::data::{ 
    repositories::ListTodoRepository,
    dto::TodoDto
};
use std::sync::Arc;

pub async fn list_todo_service(
    repo: Arc<impl ListTodoRepository>,
    user_id: i32,
) -> Result<Vec<TodoDto>, sqlx::Error> {
    repo.list(user_id).await
}
