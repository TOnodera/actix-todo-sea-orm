use crate::types::Result;

use super::{repository::TodoRepositoryTrait, value::todo::Todo};

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
        let updated = self.repository.update(id, title, body).await?;
        Ok(updated)
    }
    pub async fn delete(&self, id: i32) -> Result<bool> {
        let deleted = self.repository.delete(id).await?;
        Ok(deleted)
    }
}
