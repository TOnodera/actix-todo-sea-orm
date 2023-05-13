use sqlx::PgPool;

use super::todo::Todo;

pub struct TodoRepository {
    db: PgPool,
}

impl TodoRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
    pub fn create(title: &str, body: &str) {}
}
