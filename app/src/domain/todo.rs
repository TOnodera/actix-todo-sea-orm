use serde::{Deserialize, Serialize};

// Todoのデータ構造
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<u32>,
    pub title: String,
    pub body: String,
}
