use jwt_simple::prelude::{HS256Key, Claims, Duration, MACLike};

use crate::services::domain::user::UserService;

pub enum PasswordAuthenticateUserError {
    IncorrectPassword,
    UnknownError,
}

pub async fn password_authenticate_user(
    user_svc: &impl UserService,
    ra: &str,
    password: &str,
) -> Result<String, PasswordAuthenticateUserError> {
    let is_valid = user_svc
        .login(ra, password)
        .await
        .map_err(|_| PasswordAuthenticateUserError::UnknownError)?;

    if is_valid {
        let keys = HS256Key::generate();
        let claims = Claims::create(Duration::from_secs(2600));
        let token = keys
            .authenticate(claims)
            .map_err(|_| PasswordAuthenticateUserError::UnknownError)?;


        // TODO(edu): parei aqui, precisa catar as keys de verdade e
        // escrever o controller pra isso
        Ok(token)
    } else {
        Err(PasswordAuthenticateUserError::IncorrectPassword)
    }
}
