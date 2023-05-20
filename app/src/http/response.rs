use chrono::{DateTime, FixedOffset};
use entity::todos;
use serde::{Deserialize, Serialize};

use crate::domain::todo::Todo;

// Post /todo Response
#[derive(Deserialize, Serialize)]
pub struct PostTodoResponse {
    pub id: i32,
}
impl PostTodoResponse {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

// Get /Todo/{id} Response
#[derive(Deserialize, Serialize)]
pub struct GetTodoResponse {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
impl GetTodoResponse {
    pub fn new(
        id: i32,
        title: &str,
        body: &str,
        created_at: DateTime<FixedOffset>,
        updated_at: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            id,
            title: title.to_string(),
            body: body.to_string(),
            created_at,
            updated_at,
        }
    }
}
impl From<todos::Model> for GetTodoResponse {
    fn from(todo: todos::Model) -> Self {
        GetTodoResponse::new(
            todo.id,
            &todo.title,
            &todo.body,
            todo.created_at,
            todo.updated_at,
        )
    }
}
