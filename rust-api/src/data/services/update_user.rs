use serde::Deserialize;

use crate::data::repositories::UpdateUserRepository;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub name: String,
    pub email: String,
}

pub async fn update_user_service(
    repo: Arc<impl UpdateUserRepository>,
    user_id: i32,
    new_user: UpdateUserDto,
) -> Result<(), sqlx::Error> {
    repo.update(user_id, new_user).await
}
