use chrono::FixedOffset;
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};

// Todoのデータ構造
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub created_at: Option<chrono::DateTime<FixedOffset>>,
    pub updated_at: Option<chrono::DateTime<FixedOffset>>,
}
