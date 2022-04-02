use std::sync::Arc;

use color_eyre::eyre::Result;
use controllers::login::password_authenticate_user_ctrl;
use services::infra::{
    mail::{LettreMailSenderService, UFABCMailAddressValidationService},
    user::SqlxUserService,
    validation::SqlxValidationRequestService,
};
use simple_logger::SimpleLogger;
use sqlx::PgPool;
use warp::Filter;

mod commands;
mod controllers;
mod services;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    SimpleLogger::new().init()?;

    dotenv::dotenv()?;

    let pool = Arc::new(PgPool::connect(&std::env::var("DATABASE_URL")?).await?);

    let validation_service = Arc::new(SqlxValidationRequestService::new(pool.clone()));
    let email_service = Arc::new(LettreMailSenderService::new(
        &std::env::var("SMTP_URL")?,
        "blabla",
        "a1b1c1d1",
    )?);
    let addr_service = Arc::new(UFABCMailAddressValidationService);
    let user_service = Arc::new(SqlxUserService::new(pool.clone()));

    let send_code = crate::controllers::registration::create_validation_request_ctrl::create_filter(
        validation_service.clone(),
        email_service.clone(),
        addr_service.clone(),
    );

    let register = crate::controllers::registration::register_user_ctrl::create_filter(
        validation_service.clone(),
        user_service.clone(),
    );

    let password_login = password_authenticate_user_ctrl::create_filter(
        user_service.clone(),
    );

    let registration = warp::path("registration").and(send_code.or(register));
    let login = warp::path("login").and(password_login);

    let app = warp::path("v1").and(registration.or(login));

    warp::serve(app).run(([0, 0, 0, 0], 1234)).await;

    Ok(())
}
