use std::sync::Arc;

use crate::data::repositories::DeleteUserRepository;

pub async fn delete_user_service(
    repo: Arc<impl DeleteUserRepository>,
    user_id: i32,
) -> Result<(), sqlx::Error> {
    repo.delete(user_id).await
}
