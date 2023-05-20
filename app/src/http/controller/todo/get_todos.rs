use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    domain::{repository::TodoRepositoryTrait, AppState},
    http::{response::error_response, response::GetTodoResponse},
    infrastructure::resporitory::TodoRepository,
};

/// Get /todos
/// todo一覧を返す
#[get("/todos")]
async fn handler(data: web::Data<AppState>) -> impl Responder {
    let repository = TodoRepository::new(data.db.clone());
    match repository.get_all().await {
        Ok(models) => {
            let todos = models
                .into_iter()
                .map(|todo| todo.into())
                .collect::<Vec<GetTodoResponse>>();
            HttpResponse::Ok().json(todos)
        }
        Err(e) => error_response(e),
    }
}
