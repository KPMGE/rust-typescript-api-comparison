use crate::data::{dto::UserDto, repositories::ListUserRepository};
use std::sync::Arc;

pub async fn list_user_service(
    repo: Arc<impl ListUserRepository>,
) -> Result<Vec<UserDto>, sqlx::Error> {
    repo.list().await
}
