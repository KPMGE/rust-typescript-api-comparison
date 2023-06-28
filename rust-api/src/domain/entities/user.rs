use serde::Deserialize;

const MIN_PASSWORD_LENGTH: usize = 5;
const MIN_USER_NAME_LENGH: usize = 5;

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub enum ParseUserError {
    InvalidUserNameLength(String),
    InvalidPasswordLength(String),
}

impl User {
    pub fn parse(name: String, email: String, password: String) -> Result<Self, ParseUserError> {
        if name.len() < MIN_USER_NAME_LENGH {
            return Err(ParseUserError::InvalidUserNameLength(format!(
                "password must have at least {} characters",
                MIN_PASSWORD_LENGTH
            )));
        }

        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(ParseUserError::InvalidPasswordLength(format!(
                "user name must have at least {} characters",
                MIN_PASSWORD_LENGTH
            )));
        }

        Ok(Self {
            name,
            email,
            password,
        })
    }
}
