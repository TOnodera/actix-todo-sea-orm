use actix_web::web;
use chrono::FixedOffset;
use entity::todos;
use serde::{Deserialize, Serialize};

// Todoのデータ構造
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: chrono::DateTime<FixedOffset>,
    pub updated_at: chrono::DateTime<FixedOffset>,
}

impl From<web::Json<Todo>> for Todo {
    fn from(todo: web::Json<Todo>) -> Self {
        Self {
            id: todo.id,
            title: todo.title.to_string(),
            body: todo.body.to_string(),
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}

impl From<todos::Model> for Todo {
    fn from(todo: todos::Model) -> Self {
        Self {
            id: todo.id,
            title: todo.title.to_string(),
            body: todo.body.to_string(),
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}
