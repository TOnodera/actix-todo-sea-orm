use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PostTodoResponse {
    pub id: i32,
}
impl PostTodoResponse {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}
