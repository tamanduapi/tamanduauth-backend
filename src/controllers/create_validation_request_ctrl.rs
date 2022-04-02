use std::sync::Arc;

use serde::Deserialize;
use warp::{Filter, filters::BoxedFilter, hyper::StatusCode};

use crate::{commands::create_registration_request::{CreateRegistrationRequestErrors, create_registration_request}, services::{domain::{MailAddressValidationService, MailSenderService, ValidationRequestService}, infra::{LettreMailSenderService, SqlxValidationRequestService}}};

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
) -> impl Filter<Extract = (Arc<impl MailAddressValidationService>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || svc.clone())
}

pub fn create_filter(
    validation_svc: Arc<impl ValidationRequestService>,
    mail_svc: Arc<impl MailSenderService>,
    addr_svc: Arc<impl MailAddressValidationService>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("register")
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
	let res = create_registration_request(
		&*validation_svc,
		&*mail_svc,
        &*addr_svc,
		&body.email,
	).await;

	match res {
		Ok(_) => Ok(StatusCode::NO_CONTENT),
		Err(CreateRegistrationRequestErrors::UnauthorizedEmail) => Ok(StatusCode::UNAUTHORIZED),
		Err(CreateRegistrationRequestErrors::UnknownError) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
	}
}
