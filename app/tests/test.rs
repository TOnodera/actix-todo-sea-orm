mod utils;

#[cfg(test)]
mod tests {
    use crate::utils;
    use actix_web::{test, web, App};
    use app::{
        configure,
        domain::{todo::Todo, AppState},
    };

    #[actix_web::test]
    async fn 正常系ユースケーステスト() {
        let (tz, db) = utils::setup().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { db: db.clone(), tz }))
                .configure(configure::config),
        )
        .await;

        let request = test::TestRequest::get().uri("/todos").to_request();
        let response: Vec<Todo> = test::call_and_read_body_json(&app, request).await;

        utils::tear_down(&db).await;
    }
}
