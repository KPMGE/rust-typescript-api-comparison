use crate::{
    data::repositories::CreateUserRepository,
    domain::entities::{ParseUserError, User},
};
use email_address::EmailAddress;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    message: String,
}

#[derive(Debug)]
pub enum CreateUserError {
    InvalidUserError(ErrorMessage),
    InvalidEmailError(ErrorMessage),
    RepoError(ErrorMessage),
}

use CreateUserError::*;

pub async fn create_user_service(
    user: &User,
    repo: Arc<impl CreateUserRepository>,
) -> Result<(), CreateUserError> {
    let parsed_user = User::parse(user.name.clone(), user.email.clone(), user.password.clone())
        .map_err(|e| {
            eprintln!("ERROR: {:?}", e);
            match e {
                ParseUserError::InvalidPasswordLength(str) => {
                    InvalidUserError(ErrorMessage { message: str })
                }
                ParseUserError::InvalidUserNameLength(str) => {
                    InvalidUserError(ErrorMessage { message: str })
                }
            }
        })?;

    if !EmailAddress::is_valid(user.email.as_str()) {
        return Err(InvalidEmailError(ErrorMessage {
            message: "invalid email address".to_string(),
        }));
    }

    repo.create(&parsed_user).await.map_err(|e| {
        eprintln!("ERROR: {:?}", e);

        RepoError({
            ErrorMessage {
                message: "internal server error".to_string(),
            }
        })
    })?;

    Ok(())
}
