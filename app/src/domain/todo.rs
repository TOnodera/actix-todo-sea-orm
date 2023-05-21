use crate::types::{ApplicationError, Result};

use super::{repository::TodoRepositoryTrait, value::todo::Todo};

pub const MAX_TITLE_LENGTH: usize = 24;
pub const MAX_BODY_LENGTH: usize = 1024;

pub struct TodoDomain<T>
where
    T: TodoRepositoryTrait,
{
    pub repository: T,
}

/**
 * ドメインロジックは抽象で実装
 */
impl<T> TodoDomain<T>
where
    T: TodoRepositoryTrait,
{
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
    pub async fn create(&self, title: &str, body: &str) -> Result<i32> {
        if title.chars().count() == 0 {
            return Err(ApplicationError::DomainError(String::from(
                "タイトルを入力して下さい。",
            )));
        }
        if MAX_TITLE_LENGTH < title.chars().count() {
            return Err(ApplicationError::DomainError(format!(
                "タイトルは{}文字以内で入力して下さい。",
                MAX_TITLE_LENGTH
            )));
        }
        if MAX_BODY_LENGTH < body.chars().count() {
            return Err(ApplicationError::DomainError(format!(
                "本文は{}文字以内で入力して下さい。",
                MAX_BODY_LENGTH
            )));
        }
        let id = self.repository.create(title, body).await?;
        Ok(id)
    }
    pub async fn get(&self, id: i32) -> Result<Option<Todo>> {
        let result = self.repository.get(id).await?;
        if let Some(model) = result {
            let todo = Todo::from(model);
            return Ok(Some(todo));
        }
        Ok(None)
    }
    pub async fn get_all(&self) -> Result<Vec<Todo>> {
        let models = self.repository.get_all().await?;
        let todos = models.into_iter().map(|model| Todo::from(model)).collect();
        Ok(todos)
    }
    pub async fn update(&self, id: i32, title: &str, body: &str) -> Result<bool> {
        if MAX_TITLE_LENGTH < title.chars().count() {
            return Err(ApplicationError::DomainError(format!(
                "タイトルは{}文字以内で入力して下さい。",
                MAX_BODY_LENGTH
            )));
        }
        let updated = self.repository.update(id, title, body).await?;
        Ok(updated)
    }
    pub async fn delete(&self, id: i32) -> Result<bool> {
        let deleted = self.repository.delete(id).await?;
        Ok(deleted)
    }
}
