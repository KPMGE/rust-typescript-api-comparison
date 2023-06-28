use std::sync::Arc;

use crate::{data::repositories::CreateTodoRepository, domain::entities::Todo};

pub async fn create_todo_service(
    repo: Arc<impl CreateTodoRepository>,
    todo: Todo,
    user_id: i32
) -> Result<(), sqlx::Error> {
    repo.create(todo, user_id).await
}
