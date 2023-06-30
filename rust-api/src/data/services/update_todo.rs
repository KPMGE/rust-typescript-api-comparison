use std::sync::Arc;

use crate::data::repositories::{UpdateTodoDto, UpdateTodoRepository};

pub async fn update_todo_service(
    repo: Arc<impl UpdateTodoRepository>,
    new_todo: &UpdateTodoDto,
    todo_id: i32,
) -> Result<(), sqlx::Error> {
    repo.update(new_todo, todo_id).await
}
