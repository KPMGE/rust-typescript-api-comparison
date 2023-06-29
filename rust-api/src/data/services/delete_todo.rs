use std::sync::Arc;

use crate::data::repositories::DeleteTodoRepository;

pub async fn delete_todo_service(
    repo: Arc<impl DeleteTodoRepository>,
    todo_id: i32,
) -> Result<(), sqlx::Error> {
    repo.delete(todo_id).await
}
