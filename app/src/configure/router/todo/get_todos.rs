use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};

use crate::domain::AppState;

/// Get /todos
/// todo一覧を返す
#[get("/todos")]
async fn handler(data: web::Data<AppState>) -> impl Responder {
    #[derive(sqlx::FromRow)]
    struct Date {
        pub hoge: String,
    }

    match sqlx::query_as::<_, Date>("SELECT 'hoge' as hoge")
        .fetch_one(&data.db)
        .await
    {
        Ok(row) => {
            return HttpResponse::Ok().body(format!("/done/{}", row.hoge));
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    HttpResponse::Ok().body("/done")
}
