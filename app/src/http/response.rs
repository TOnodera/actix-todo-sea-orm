use actix_web::HttpResponse;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{domain::value::todo::Todo, logger, types::ApplicationError};

// Post /todo Response
#[derive(Deserialize, Serialize)]
pub struct PostTodoResponse {
    pub id: i32,
}
impl PostTodoResponse {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

// Get /Todo/{id} Response
#[derive(Deserialize, Serialize)]
pub struct GetTodoResponse {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
impl GetTodoResponse {
    pub fn new(
        id: i32,
        title: &str,
        body: &str,
        created_at: DateTime<FixedOffset>,
        updated_at: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            id,
            title: title.to_string(),
            body: body.to_string(),
            created_at,
            updated_at,
        }
    }
}
impl From<Todo> for GetTodoResponse {
    fn from(todo: Todo) -> Self {
        GetTodoResponse::new(
            todo.id,
            &todo.title,
            &todo.body,
            todo.created_at,
            todo.updated_at,
        )
    }
}

// リポジトリ層、ドメイン層のエラーをレスポンスに変換
pub fn error_response(error: ApplicationError) -> HttpResponse {
    match error {
        // ドメインエラーならエラーメッセージ返却
        ApplicationError::DomainError(message) => {
            HttpResponse::BadRequest().json(ErrorMessage::new(message))
        }
        e => {
            logger::error(&format!("{}", e));
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ErrorMessage {
    pub message: String,
}
impl ErrorMessage {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
