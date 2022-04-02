use async_trait::async_trait;
use color_eyre::Result;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

use crate::services::domain::mail::{MailAddressValidationService, MailSenderService};

pub struct LettreMailSenderService {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl LettreMailSenderService {
    pub fn new(url: &str, username: &str, password: &str) -> Result<LettreMailSenderService> {
        let _credentials = Credentials::new(username.to_owned(), password.to_owned());

        // TODO use actual server
        //let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(url)?
        //  .credentials(credentials)
        //  .build();
        let transport = AsyncSmtpTransport::<Tokio1Executor>::unencrypted_localhost();

        Ok(LettreMailSenderService { transport })
    }
}

#[async_trait]
impl MailSenderService for LettreMailSenderService {
    async fn send_mail(&self, email: &str, subject: &str, message: String) -> Result<()> {
        let message = Message::builder()
            .from(
                "Tamanduauth <tamanduauth@authmail.tamanduapi.gay>"
                    .parse()
                    .unwrap(),
            )
            .to(email.parse().unwrap())
            .subject(subject)
            .body(message)?;

        self.transport.send(message).await?;

        Ok(())
    }
}

pub struct UFABCMailAddressValidationService;

impl MailAddressValidationService for UFABCMailAddressValidationService {
    fn is_valid(&self, email: &str) -> bool {
        email.ends_with("ufabc.edu.br")
    }
}
