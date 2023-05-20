mod utils;

#[cfg(test)]
mod tests {
    use crate::utils;
    use actix_web::{test, web, App};
    use app::{
        configure,
        domain::{todo::Todo, AppState},
        http::response::PostTodoResponse,
    };
    use serde_json::json;

    #[actix_web::test]
    async fn 正常系ユースケーステスト() {
        let (tz, db) = utils::setup().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { db: db.clone(), tz }))
                .configure(configure::config),
        )
        .await;

        //
        // Post Todo 新規登録
        // 登録
        //
        let new_post = json!({
            "title": "新規登録",
            "body": "新規でTODOを登録しました。"
        });
        let req = test::TestRequest::post()
            .uri("/todo")
            .set_json(new_post)
            .to_request();
        let response: PostTodoResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(response.id, 1);
        // id 指定でデータを取得する
        let req = test::TestRequest::get().uri("/todo/1").to_request();
        let response: Todo = test::call_and_read_body_json(&app, req).await;
        assert_eq!(&response.title, "新規登録");
        assert_eq!(&response.body, "新規でTODOを登録しました。");

        //
        // 更新
        // Patch Todo 更新
        //
        let update_post = json!({
            "title": "更新登録",
            "body": "更新でTODOを登録しました。"

        });
        let req = test::TestRequest::patch()
            .uri("/todo/1")
            .set_json(update_post)
            .to_request();
        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());
        // id 指定でデータを取得する
        let req = test::TestRequest::get().uri("/todo/1").to_request();
        let response: Todo = test::call_and_read_body_json(&app, req).await;
        assert_eq!(&response.title, "更新登録");
        assert_eq!(&response.body, "更新でTODOを登録しました。");

        //
        // 削除
        // Delete Post 削除
        //
        let req = test::TestRequest::delete().uri("/todo/1").to_request();
        let response = test::call_service(&app, req).await;
        assert!(response.status().is_success());

        //
        // 一覧取得
        // Get Todos 一覧取得
        //
        let todos = vec![
            json!({
                "title": "title1",
                "body": "body1"
            }),
            json!({
                "title": "title2",
                "body": "body2"
            }),
            json!({
                "title": "title3",
                "body": "body3"
            }),
        ];
        for todo in todos {
            let req = test::TestRequest::post()
                .uri("/todo")
                .set_json(todo)
                .to_request();
            let response = test::call_service(&app, req).await;
            assert!(response.status().is_success());
        }
        // 一覧取得
        let req = test::TestRequest::get().uri("/todos").to_request();
        let response: Vec<Todo> = test::call_and_read_body_json(&app, req).await;

        utils::tear_down(&db).await;
    }
}
