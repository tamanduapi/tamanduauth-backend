use async_trait::async_trait;
use color_eyre::Result;

pub struct ValidationRequest {
    pub id: uuid::Uuid,
    pub email: String,
    pub hashed_code: String,
}

impl ValidationRequest {
    pub fn new(email: String) -> ValidationRequest {
        ValidationRequest {
            id: uuid::Uuid::new_v4(),
            hashed_code: uuid::Uuid::new_v4().to_string(),
            email,
        }
    }
}

#[async_trait]
pub trait ValidationRequestService: Send + Sync {
    async fn has_in_flight(&self, email: &str) -> Result<bool>;
    async fn save(&self, request: &ValidationRequest) -> Result<()>;
    async fn validate(&self, email: &str, code: &str) -> Result<bool>;
}
