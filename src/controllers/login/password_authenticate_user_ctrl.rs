use std::sync::Arc;

use serde::{Deserialize, Serialize};
use warp::{Filter, hyper::StatusCode};

use crate::{services::domain::user::UserService, commands::login::password_authenticate_user::{password_authenticate_user, PasswordAuthenticateUserError}};

#[derive(Deserialize)]
struct PasswordAuthenticateUserRequest {
    pub ra: String,
    pub password: String,
}

#[derive(Serialize)]
struct PasswordAuthenticateUserResponse {
    pub token: String,
}

fn with_user_svc(
    svc: Arc<impl UserService>,
) -> impl Filter<Extract = (Arc<impl UserService>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || svc.clone())
}

pub fn create_filter(
    user_svc: Arc<impl UserService>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("password-login")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json())
        .and(with_user_svc(user_svc))
        .and_then(password_authenticate_user_handler)
}

async fn password_authenticate_user_handler(
    body: PasswordAuthenticateUserRequest,
    user_svc: Arc<impl UserService>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match password_authenticate_user(&*user_svc, &body.ra, &body.password).await {
        Ok(token) => {
            let response = PasswordAuthenticateUserResponse { token };
            let reply = warp::reply::json(&response);

            Ok(Box::new(warp::reply::with_status(reply, StatusCode::OK)))
        }
        Err(PasswordAuthenticateUserError::IncorrectPassword) => Ok(Box::new(StatusCode::UNAUTHORIZED)),
        Err(PasswordAuthenticateUserError::UnknownError) => Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
