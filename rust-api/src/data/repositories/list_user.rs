use crate::data::dto::UserDto;
use async_trait::async_trait;

#[async_trait]
pub trait ListUserRepository {
    async fn list(&self) -> Result<Vec<UserDto>, sqlx::Error>;
}
