use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub name: String,
    pub email: String,
}
