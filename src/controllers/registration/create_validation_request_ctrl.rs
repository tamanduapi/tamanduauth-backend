use std::sync::Arc;

use serde::Deserialize;
use warp::{hyper::StatusCode, Filter};

use crate::{
    commands::registration::create_registration_request::{
        create_registration_request, CreateRegistrationRequestErrors,
    },
    services::domain::{
        mail::{MailAddressValidationService, MailSenderService},
        validation::ValidationRequestService,
    },
};

#[derive(Deserialize, Debug)]
pub struct CreateValidationRequestBody {
    pub email: String,
}

fn with_validation_svc(
    svc: Arc<impl ValidationRequestService>,
) -> impl Filter<Extract = (Arc<impl ValidationRequestService>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || svc.clone())
}

fn with_mail_svc(
    svc: Arc<impl MailSenderService>,
) -> impl Filter<Extract = (Arc<impl MailSenderService>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || svc.clone())
}

fn with_addr_svc(
    svc: Arc<impl MailAddressValidationService>,
) -> impl Filter<Extract = (Arc<impl MailAddressValidationService>,), Error = std::convert::Infallible>
       + Clone {
    warp::any().map(move || svc.clone())
}

pub fn create_filter(
    validation_svc: Arc<impl ValidationRequestService>,
    mail_svc: Arc<impl MailSenderService>,
    addr_svc: Arc<impl MailAddressValidationService>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("send-code")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json())
        .and(with_validation_svc(validation_svc))
        .and(with_mail_svc(mail_svc))
        .and(with_addr_svc(addr_svc))
        .and_then(create_validation_request)
}

pub async fn create_validation_request<'a>(
    body: CreateValidationRequestBody,
    validation_svc: Arc<impl ValidationRequestService>,
    mail_svc: Arc<impl MailSenderService>,
    addr_svc: Arc<impl MailAddressValidationService>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res =
        create_registration_request(&*validation_svc, &*mail_svc, &*addr_svc, &body.email).await;

    match res {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(CreateRegistrationRequestErrors::UnauthorizedEmail) => Ok(StatusCode::UNAUTHORIZED),
        Err(CreateRegistrationRequestErrors::UnknownError) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
