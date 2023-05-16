use crate::types::{ApplicationError, Result};

pub struct TodoRepository<'a> {
    db: &'a PgPool,
}

impl<'a> TodoRepository<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }
    pub async fn create(&self, title: &str, body: &str) -> Result<i32> {
        // トランザクション開始
        let begin = self.db.begin().await;
        let tx = match begin {
            Ok(tx) => tx,
            Err(e) => {
                return Err(ApplicationError::DatabaseError(String::from(
                    "トランザクションの開始に失敗しました。",
                )));
            }
        };

        // 登録処理
        let insert = sqlx::query!(
            "INSERT INTO todos (title , body) VALUES ( $1, $2) RETURNING id",
            title,
            body
        )
        .fetch_one(self.db)
        .await;
        let id = match insert {
            Ok(record) => record.id,
            Err(e) => {
                return Err(ApplicationError::DatabaseError(String::from(
                    "登録に失敗しました。",
                )))
            }
        };

        // コミット
        if let Err(e) = tx.commit().await {
            return Err(ApplicationError::DatabaseError(String::from(
                "コミットできませんでした。",
            )));
        };

        Ok(id)
    }
}
