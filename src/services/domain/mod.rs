use async_trait::async_trait;
use color_eyre::eyre::Result;

// TODO: move each service to its own file

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

#[async_trait]
pub trait MailSenderService: Send + Sync {
    async fn send_mail(&self, email: &str, subject: &str, message: String) -> Result<()>;
}

pub trait MailAddressValidationService: Send + Sync {
    fn is_valid(&self, email: &str) -> bool;
}

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
}
