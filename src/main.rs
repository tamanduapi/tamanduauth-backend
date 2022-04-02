use std::sync::Arc;

use color_eyre::eyre::Result;
use services::infra::{LettreMailSenderService, SqlxValidationRequestService, UFABCMailAddressValidationService};
use simple_logger::SimpleLogger;
use sqlx::PgPool;

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

    //create_registration_request(
    //  &validation_service,
    //  &email_service,
    //  "aaa@bbb.com"
    //).await?;

    //let app = warp::path("v1")
    //  .or();

    let app = crate::controllers::create_validation_request_ctrl::create_filter(
        validation_service.clone(),
        email_service.clone(),
        addr_service.clone(),
    );

    warp::serve(app).run(([0, 0, 0, 0], 1234)).await;

    Ok(())
}
