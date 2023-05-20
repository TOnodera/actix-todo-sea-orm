use serde::{Deserialize, Serialize};

// Post Todo
#[derive(Serialize, Deserialize)]
pub struct PostTodoRequest {
    pub title: String,
    pub body: String,
}

// Patch Todo
#[derive(Serialize, Deserialize)]
pub struct PatchTodoRequest {
    pub title: String,
    pub body: String,
}
