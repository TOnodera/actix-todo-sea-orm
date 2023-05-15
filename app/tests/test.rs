#[cfg(test)]
mod tests {
    use actix_web::{get, test, web, App, HttpResponse, Responder};
    use app::configure;

    #[get("/")]
    async fn index() -> impl Responder {
        HttpResponse::Ok().body("test")
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route(
            "/",
            web::get().to(|| async {
                return HttpResponse::Ok().body("test");
            }),
        ))
        .await;

        let request = test::TestRequest::default().to_request();
        let response = test::call_service(&app, request).await;
        assert!(response.status().is_success());
    }
}
