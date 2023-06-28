use crate::domain::entities::User;
use async_trait::async_trait;

#[async_trait]
pub trait CreateUserRepository {
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;
}
