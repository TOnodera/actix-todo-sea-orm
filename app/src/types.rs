use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: Option<u32>,
    name: String,
    body: Option<String>
}