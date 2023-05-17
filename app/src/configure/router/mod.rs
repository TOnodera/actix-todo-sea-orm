use actix_web::{web::ServiceConfig, HttpResponse};
use serde_json::json;

use crate::types::ApplicationError;

mod todo;

pub fn route(cfg: &mut ServiceConfig) {
    todo::todos_route(cfg);
}

// リポジトリ層、ドメイン層のエラーをレスポンスに変換
pub fn error_response(error: ApplicationError) -> HttpResponse {
    match error {
        // ドメインエラーならエラーメッセージ返却
        ApplicationError::DomainError(message) => {
            HttpResponse::BadRequest().json(json!({ "message": message }))
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}
