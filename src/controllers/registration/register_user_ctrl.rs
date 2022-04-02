use std::sync::Arc;

use serde::Deserialize;
use warp::{hyper::StatusCode, Filter};

use crate::{
    commands::registration::register_user::{register_user, RegisterUserErrors},
    services::domain::{
        user::{User, UserService},
        validation::ValidationRequestService,
    },
};

#[derive(Deserialize)]
struct RegisterUserRequestBody {
    pub password: String,
    pub ra: String,
    pub email: String,
    pub hashed_code: String,
}

impl Into<User> for RegisterUserRequestBody {
    fn into(self) -> User {
        User {
            id: uuid::Uuid::new_v4(),
            password: bcrypt::hash(self.password, bcrypt::DEFAULT_COST).unwrap(),
            ra: self.ra,
            email: self.email,
        }
    }
}

fn with_validation_svc(
    svc: Arc<impl ValidationRequestService>,
) -> impl Filter<Extract = (Arc<impl ValidationRequestService>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || svc.clone())
}

fn with_user_svc(
    svc: Arc<impl UserService>,
) -> impl Filter<Extract = (Arc<impl UserService>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || svc.clone())
}

pub fn create_filter(
    validation_svc: Arc<impl ValidationRequestService>,
    user_svc: Arc<impl UserService>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("register")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json())
        .and(with_validation_svc(validation_svc))
        .and(with_user_svc(user_svc))
        .and_then(register_user_handler)
}

async fn register_user_handler(
    body: RegisterUserRequestBody,
    validation_svc: Arc<impl ValidationRequestService>,
    user_svc: Arc<impl UserService>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_code = body.hashed_code.clone();
    let user: User = body.into();

    match register_user(&*validation_svc, &*user_svc, &hashed_code, &user).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(RegisterUserErrors::UserExists) => Ok(StatusCode::UNPROCESSABLE_ENTITY),
        Err(RegisterUserErrors::InvalidCode) => Ok(StatusCode::UNAUTHORIZED),
        Err(RegisterUserErrors::UnknownError) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
