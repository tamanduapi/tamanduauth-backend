use crate::services::domain::MailAddressValidationService;
use crate::services::domain::MailSenderService;
use crate::services::domain::ValidationRequest;
use crate::services::domain::ValidationRequestService;

pub enum CreateRegistrationRequestErrors {
    UnauthorizedEmail,
    UnknownError,
}

pub async fn create_registration_request(
    validation_svc: &impl ValidationRequestService,
    email_svc: &impl MailSenderService,
    addr_svc: &impl MailAddressValidationService,
    email: &str,
) -> Result<(), CreateRegistrationRequestErrors> {
    if !addr_svc.is_valid(email) {
        return Err(CreateRegistrationRequestErrors::UnauthorizedEmail);
    }

    let has_in_flight = validation_svc
        .has_in_flight(email)
        .await
        .map_err(|_| CreateRegistrationRequestErrors::UnknownError)?;

    if has_in_flight {
        return Ok(());
    }

    let request = ValidationRequest::new(email.to_string());

    // TODO i18n
    let message = format!("Seu código Tamanduauth é: {}", request.hashed_code);

    email_svc
        .send_mail(email, "Código Tamanduauth", message)
        .await
        .map_err(|_| CreateRegistrationRequestErrors::UnknownError)?;

    validation_svc
        .save(&request)
        .await
        .map_err(|_| CreateRegistrationRequestErrors::UnknownError)?;

    Ok(())
}
