use actix_web::web::ServiceConfig;

mod todo;

pub fn route(cfg: &mut ServiceConfig) {
    todo::todos_route(cfg);
}
