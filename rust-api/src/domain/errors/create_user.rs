use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Debug)]
pub enum CreateUserError {
    InvalidUserError(ErrorMessage),
    InvalidEmailError(ErrorMessage),
    RepoError(ErrorMessage),
}
