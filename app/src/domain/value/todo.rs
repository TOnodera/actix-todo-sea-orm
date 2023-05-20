use chrono::FixedOffset;
use entity::todos;
use serde::{Deserialize, Serialize};

use crate::configure::tz;

// Todoのデータ構造
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: chrono::DateTime<FixedOffset>,
    pub updated_at: chrono::DateTime<FixedOffset>,
}

impl From<todos::Model> for Todo {
    fn from(todo: todos::Model) -> Self {
        Self {
            id: todo.id,
            title: todo.title.to_string(),
            body: todo.body.to_string(),
            created_at: todo.created_at.with_timezone(&tz()),
            updated_at: todo.updated_at.with_timezone(&tz()),
        }
    }
}
