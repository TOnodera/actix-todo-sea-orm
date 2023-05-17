use actix_web::web::ServiceConfig;

use crate::types::ApplicationError;

mod delete_todo;
mod get_todo;
mod get_todos;
mod patch_todo;
mod post_todo;

/// /todos,/todoのルートを設定
pub fn todos_route(cfg: &mut ServiceConfig) {
    cfg.service(get_todos::handler)
        .service(get_todo::handler)
        .service(post_todo::handler)
        .service(patch_todo::handler)
        .service(delete_todo::handler);
}
