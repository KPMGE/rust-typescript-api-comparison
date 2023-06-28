use async_trait::async_trait;

#[async_trait]
pub trait DeleteUserRepository {
    async fn delete(&self, user_id: i32) -> Result<(), sqlx::Error>;
}
