use std::error::Error;

use thiserror::Error;

pub type Result<T> = anyhow::Result<T, ApplicationError>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("起動エラー: {0}")]
    LaunchError(String),
    #[error("アプリケーションエラー: {0}")]
    DomainError(String),
    #[error("データベースエラー: {0}")]
    DatabaseError(String),
    #[error("原因不明なエラー: {0}")]
    UnknownError(String),
}
