use async_trait::async_trait;
use color_eyre::Result;

#[async_trait]
pub trait MailSenderService: Send + Sync {
    async fn send_mail(&self, email: &str, subject: &str, message: String) -> Result<()>;
}

pub trait MailAddressValidationService: Send + Sync {
    fn is_valid(&self, email: &str) -> bool;
}
