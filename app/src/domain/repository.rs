use entity::todos;
use sea_orm::entity::Set;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, RuntimeErr::SqlxError};

use crate::types::{ApplicationError, Result};

fn error_logger(error: DbErr) {
    match error {
        DbErr::Exec(SqlxError(error)) => match error {
            e => {
                // TODO ログ出せるようにする
                todo!()
            }
        },
        _ => panic!("Unexpected Error kind"),
    }
}

pub struct TodoRepository {
    db: DatabaseConnection,
}

impl TodoRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
    pub async fn create(&self, title: &str, body: &str) -> Result<i32> {
        let todo = todos::ActiveModel {
            title: Set(title.to_string()),
            body: Set(body.to_string()),
            ..Default::default()
        };

        let insert_result = todos::Entity::insert(todo)
            .exec(&self.db)
            .await
            .map_err(|e| return ApplicationError::DatabaseError)?;
        Ok(insert_result.last_insert_id)
    }
}
