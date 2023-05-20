use thiserror::Error;

pub type Result<T> = anyhow::Result<T, ApplicationError>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("起動エラー: {0}")]
    LaunchError(String),
    #[error("アプリケーションエラー: {0}")]
    DomainError(String),
    #[error("データベースエラー")]
    DatabaseError,
    #[error("IOエラー: {0}")]
    IoError(String),
    #[error("原因不明なエラー: {0}")]
    UnknownError(String),
}
