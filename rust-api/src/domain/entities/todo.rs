use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub completed: Option<bool>,
}
