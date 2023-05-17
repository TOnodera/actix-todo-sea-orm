use entity::todos;
use sea_orm::entity::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{ModelTrait, QueryOrder};

use crate::types::{ApplicationError, Result};

use super::todo::Todo;

pub struct TodoRepository {
    db: DatabaseConnection,
}

impl TodoRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // 登録
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

    // 全件取得
    pub async fn gets_all(&self) -> Result<Vec<Todo>> {
        let models = todos::Entity::find()
            .order_by_desc(todos::Column::UpdatedAt)
            .all(&self.db)
            .await
            .map_err(|e| ApplicationError::DatabaseError)?;

        let todos = models
            .into_iter()
            .map(|todo| Todo {
                id: Some(todo.id),
                title: todo.title,
                body: todo.body,
                created_at: Some(todo.created_at),
                updated_at: Some(todo.updated_at),
            })
            .collect::<Vec<Todo>>();
        Ok(todos)
    }

    // id指定でデータを取得
    pub async fn get(&self, id: i32) -> Result<Option<Todo>> {
        let model = todos::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApplicationError::DatabaseError)?;

        let todo = match model {
            Some(todo) => Some(Todo {
                id: Some(todo.id),
                title: todo.title,
                body: todo.body,
                created_at: Some(todo.created_at),
                updated_at: Some(todo.updated_at),
            }),
            None => None,
        };
        Ok(todo)
    }

    // id指定でデータを削除
    pub async fn delete(&self, id: i32) -> Result<u64> {
        let model = todos::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApplicationError::DatabaseError)?;

        if let Some(todo) = model {
            let res = todo
                .delete(&self.db)
                .await
                .map_err(|e| ApplicationError::DatabaseError)?;
            return Ok(res.rows_affected);
        }
        Ok(0)
    }
}
