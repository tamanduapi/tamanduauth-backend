use async_trait::async_trait;
use color_eyre::Result;

pub struct User {
    pub id: uuid::Uuid,
    pub password: String,
    pub ra: String,
    pub email: String,
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn user_exists(&self, email: &str) -> Result<bool>;
    async fn save_user(&self, user: &User) -> Result<()>;
    async fn login(&self, ra: &str, password: &str) -> Result<bool>;
}
