use crate::services::domain::{
    user::{User, UserService},
    validation::ValidationRequestService,
};

pub enum RegisterUserErrors {
    UserExists,
    InvalidCode,
    UnknownError,
}

pub async fn register_user(
    validation_svc: &impl ValidationRequestService,
    user_svc: &impl UserService,
    hashed_code: &str,
    user: &User,
) -> Result<(), RegisterUserErrors> {
    let is_code_valid = validation_svc
        .validate(&user.email, hashed_code)
        .await
        .map_err(|_| RegisterUserErrors::UnknownError)?;

    if !is_code_valid {
        return Err(RegisterUserErrors::InvalidCode);
    }

    let user_exists = user_svc
        .user_exists(&user.email)
        .await
        .map_err(|_| RegisterUserErrors::UnknownError)?;

    if user_exists {
        return Err(RegisterUserErrors::UserExists);
    }

    user_svc
        .save_user(user)
        .await
        .map_err(|_| RegisterUserErrors::UnknownError)?;

    Ok(())
}
