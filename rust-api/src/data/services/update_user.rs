use crate::data::{ 
    repositories::UpdateUserRepository,
    dto::UpdateUserDto
};
use std::sync::Arc;

pub async fn update_user_service(
    repo: Arc<impl UpdateUserRepository>,
    user_id: i32,
    new_user: &UpdateUserDto,
) -> Result<(), sqlx::Error> {
    repo.update(user_id, new_user).await
}
