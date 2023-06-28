use std::sync::Arc;
use crate::data::{repositories::ListUserRepository, dto::UserDto};

pub async fn list_user_service(repo: Arc<impl ListUserRepository>) -> Result<Vec<UserDto>, sqlx::Error> {
    repo.list().await
}
