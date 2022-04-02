use std::sync::Arc;

use async_trait::async_trait;
use color_eyre::eyre::Result;
use lettre::AsyncSmtpTransport;
use lettre::AsyncTransport;
use lettre::Message;
use lettre::Tokio1Executor;

use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::AsyncSmtpTransportBuilder;
use sqlx::PgPool;

use super::domain::MailAddressValidationService;
use super::domain::MailSenderService;
use super::domain::UserService;
use super::domain::ValidationRequestService;

pub struct SqlxValidationRequestService {
    pool: Arc<PgPool>,
}

impl SqlxValidationRequestService {
    pub fn new(pool: Arc<PgPool>) -> SqlxValidationRequestService {
        SqlxValidationRequestService { pool }
    }
}

#[async_trait]
impl ValidationRequestService for SqlxValidationRequestService {
    async fn has_in_flight(&self, email: &str) -> Result<bool> {
        let max_age = std::env::var("TAMANDUAUTH_VALIDATION_REQUEST_MAX_AGE")
            .map(|i| i.parse::<u64>().unwrap())
            .unwrap_or(60);

        #[allow(deprecated)]
        let t =
            sqlx::types::time::PrimitiveDateTime::now() - std::time::Duration::from_secs(max_age);

        let row = sqlx::query!(
            "SELECT validation_id FROM Validations WHERE email=$1 AND created_at > $2",
            email,
            t,
        )
        .fetch_one(&*self.pool)
        .await;

        match row {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    #[allow(deprecated)]
    async fn save(&self, request: &super::domain::ValidationRequest) -> Result<()> {
        sqlx::query!(
			"INSERT INTO Validations (validation_id, email, hashed_code, created_at) VALUES ($1, $2, $3, $4)",
			request.id,
			request.email,
			request.hashed_code,
			sqlx::types::time::PrimitiveDateTime::now(), // TODO use the saved created_at
		)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    async fn validate(&self, email: &str, code: &str) -> Result<bool> {
        let row = sqlx::query!(
            "SELECT validation_id FROM Validations WHERE email=$1 AND hashed_code=$2",
            email,
            code,
        )
            .fetch_one(&*self.pool)
            .await;

        match row {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

pub struct LettreMailSenderService {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl LettreMailSenderService {
    pub fn new(url: &str, username: &str, password: &str) -> Result<LettreMailSenderService> {
        let _credentials = Credentials::new(username.to_owned(), password.to_owned());

        // TODO use actual server
        //let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(url)?
        //	.credentials(credentials)
        //	.build();
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

pub struct SqlxUserService {
    pool: Arc<PgPool>,
}

impl SqlxUserService {
    pub fn new(pool: Arc<PgPool>) -> SqlxUserService {
        SqlxUserService { pool }
    }
}

#[async_trait]
impl UserService for SqlxUserService {
    async fn user_exists(&self, email: &str) -> Result<bool> {
        let row = sqlx::query!("SELECT user_id FROM Users WHERE email=$1", email)
            .fetch_one(&*self.pool)
            .await;

        match row {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn save_user(&self, user: &super::domain::User) -> Result<()> {
        sqlx::query!(
            "INSERT INTO Users VALUES ($1, $2, $3, $4, NOW())",
            user.id,
            user.password,
            user.ra,
            user.email,
        )
            .execute(&*self.pool)
            .await?;

        Ok(())
    }
}
