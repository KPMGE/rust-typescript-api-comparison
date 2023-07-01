use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateTodoDto {
    pub title: String,
    pub description: String,
}
