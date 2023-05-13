use sqlx::PgPool;

use crate::types::Result;

pub struct TodoRepository<'a> {
    db: &'a PgPool,
}

impl<'a> TodoRepository<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }
    pub async fn create(&self, title: &str, body: &str) -> Result<i32> {
        let tx = self.db.begin().await?;
        let res = sqlx::query!(
            "INSERT INTO todos (title , body) VALUES ( $1, $2) RETURNING id",
            title,
            body
        )
        .fetch_one(self.db)
        .await?;
        tx.commit().await?;

        Ok(res.id)
    }
}
