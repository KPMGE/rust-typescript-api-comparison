use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub email: String,
}
