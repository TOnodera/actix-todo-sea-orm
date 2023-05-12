use actix_web::{Resource, web::ServiceConfig};

mod todos;

pub fn route(cfg: &mut ServiceConfig) {
    todos::todos_route(cfg);
}