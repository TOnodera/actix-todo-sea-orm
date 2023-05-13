use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("起動エラー: {0}")]
    LaunchError(String),
    #[error("エラー: {0}")]
    DomainError(String),
    #[error("原因不明なエラー: {0}")]
    UnknownError(String),
}
