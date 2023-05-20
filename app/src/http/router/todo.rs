use actix_web::web::ServiceConfig;

use crate::http::controller;

/// /todos,/todoのルートを設定
pub fn todos_route(cfg: &mut ServiceConfig) {
    cfg.service(controller::todo::get_todos::handler)
        .service(controller::todo::get_todo::handler)
        .service(controller::todo::post_todo::handler)
        .service(controller::todo::patch_todo::handler)
        .service(controller::todo::delete_todo::handler);
}
