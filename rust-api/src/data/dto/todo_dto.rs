use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TodoDto {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
