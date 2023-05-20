use async_trait::async_trait;
use entity::todos;

use crate::types::Result;

#[async_trait]
pub trait TodoRepositoryTrait {
    async fn create(&self, title: &str, body: &str) -> Result<i32>;
    async fn get(&self, id: i32) -> Result<Option<todos::Model>>;
    async fn get_all(&self) -> Result<Vec<todos::Model>>;
    async fn update(&self, id: i32, title: &str, body: &str) -> Result<bool>;
    async fn delete(&self, id: i32) -> Result<bool>;
}
