use crate::services::domain::{User, UserService, ValidationRequestService};

pub enum RegisterUserErrors {
    UserExists,
    InvalidCode,
}

pub async fn register_user(
    validation_svc: &impl ValidationRequestService,
    user_svc: &impl UserService,
    hashed_code: &str,
    user: &User,
) -> Result<(), RegisterUserErrors> {
    Ok(())
}
