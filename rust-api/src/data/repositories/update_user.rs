use crate::data::services::UpdateUserDto;
use async_trait::async_trait;

#[async_trait]
pub trait UpdateUserRepository {
    async fn update(&self, user_id: i32, user: UpdateUserDto) -> Result<(), sqlx::Error>;
}
