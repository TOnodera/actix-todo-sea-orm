use actix_web::cookie::time::util;
use chrono::format::Fixed;
use chrono::{FixedOffset, Local, TimeZone, Utc};
use entity::todos;
use sea_orm::entity::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use sea_orm::{ModelTrait, QueryOrder};

use crate::types::{ApplicationError, Result};
use crate::{configure, utils};

use super::todo;

pub struct TodoRepository {
    db: DatabaseConnection,
    tz: FixedOffset,
}

impl TodoRepository {
    pub fn new(db: DatabaseConnection, tz: FixedOffset) -> Self {
        Self { db, tz }
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
    pub async fn gets_all(&self) -> Result<Vec<todos::Model>> {
        let models = todos::Entity::find()
            .order_by_desc(todos::Column::UpdatedAt)
            .all(&self.db)
            .await
            .map_err(|e| ApplicationError::DatabaseError)?;

        Ok(models)
    }

    // id指定でデータを取得
    pub async fn get(&self, id: i32) -> Result<Option<todos::Model>> {
        let model = todos::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApplicationError::DatabaseError)?;

        Ok(model)
    }

    // id指定でデータを更新
    pub async fn update(&self, id: i32, title: &str, body: &str) -> Result<()> {
        let model = todos::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApplicationError::DatabaseError)?;

        if let Some(m) = model {
            let mut old_model: todos::ActiveModel = m.into();
            old_model.title = Set(title.to_string());
            old_model.body = Set(body.to_string());
            old_model.updated_at = Set(utils::now()?);
            old_model
                .update(&self.db)
                .await
                .map_err(|e| ApplicationError::DatabaseError)?;
        }
        Ok(())
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
