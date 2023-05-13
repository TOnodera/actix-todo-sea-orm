use actix_web::web::ServiceConfig;

mod todos;

pub fn route(cfg: &mut ServiceConfig) {
    todos::todos_route(cfg);
}
