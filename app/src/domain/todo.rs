use serde::{Serialize, Deserialize};

// Todoのデータ構造
#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: Option<u32>,
    name: String,
    body: String
}