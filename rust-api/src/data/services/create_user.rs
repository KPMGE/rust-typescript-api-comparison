use crate::{
    data::repositories::CreateUserRepository,
    domain::entities::{ParseUserError, User},
    domain::errors::{CreateUserError, ErrorMessage},
};
use email_address::EmailAddress;
use std::sync::Arc;

pub async fn create_user_service(
    user: &User,
    repo: Arc<impl CreateUserRepository>,
) -> Result<(), CreateUserError> {
    let parsed_user = User::parse(user.name.clone(), user.email.clone(), user.password.clone())
        .map_err(|e| {
            eprintln!("ERROR: {:?}", e);
            match e {
                ParseUserError::InvalidPasswordLength(str) => {
                    CreateUserError::InvalidUserError(ErrorMessage { message: str })
                }
                ParseUserError::InvalidUserNameLength(str) => {
                    CreateUserError::InvalidUserError(ErrorMessage { message: str })
                }
            }
        })?;

    if !EmailAddress::is_valid(user.email.as_str()) {
        return Err(CreateUserError::InvalidEmailError(ErrorMessage {
            message: "invalid email address".to_string(),
        }));
    }

    repo.create(&parsed_user).await.map_err(|e| {
        eprintln!("ERROR: {:?}", e);

        CreateUserError::RepoError({
            ErrorMessage {
                message: "internal server error".to_string(),
            }
        })
    })?;

    Ok(())
}
