mod utils;

#[cfg(test)]
mod tests {
    use crate::utils;
    use actix_web::{test, web, App};
    use app::{
        configure,
        domain::{todo::MAX_BODY_LENGTH, AppState},
        http::response::{ErrorMessage, GetTodoResponse, PostTodoResponse},
    };
    use serde_json::json;
    use tokio::time::{sleep, Duration};

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
        let response: GetTodoResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!(&response.title, "新規登録");
        assert_eq!(&response.body, "新規でTODOを登録しました。");

        // タイムゾーン設定が正しいことを確認する
        let tz = response.created_at.timezone();
        let diff = tz.utc_minus_local();
        assert_eq!(diff, -9 * 3600);
        let tz = response.updated_at.timezone();
        let diff = tz.utc_minus_local();
        assert_eq!(diff, -9 * 3600);

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
        let response: GetTodoResponse = test::call_and_read_body_json(&app, req).await;
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
            sleep(Duration::from_secs(1)).await;
        }
        // 一覧取得
        let req = test::TestRequest::get().uri("/todos").to_request();
        let response: Vec<GetTodoResponse> = test::call_and_read_body_json(&app, req).await;

        // 登録新しい順にデータが返ってくること
        assert_eq!(&response[0].title, "title3");
        assert_eq!(&response[1].title, "title2");
        assert_eq!(&response[2].title, "title1");

        utils::tear_down(&db).await;
    }

    #[actix_web::test]
    async fn 異常系ユースケーステスト() {
        let (tz, db) = utils::setup().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { db: db.clone(), tz }))
                .configure(configure::config),
        )
        .await;

        //
        // Post Todo 新規登録
        // タイトル未入力
        //
        let post = json!({
            "title": "",
            "body": "新規でTODOを登録しました。"
        });
        let req = test::TestRequest::post()
            .uri("/todo")
            .set_json(post)
            .to_request();

        let response: ErrorMessage = test::call_and_read_body_json(&app, req).await;
        assert_eq!(response.message, "タイトルを入力して下さい。");

        //
        // Post Todo 新規登録
        // 本文文字数境界値テスト
        //
        let post = json!({
            "title": "タイトル",
            "body": "文".repeat(MAX_BODY_LENGTH)
        });
        let req = test::TestRequest::post()
            .uri("/todo")
            .set_json(post)
            .to_request();
        let response: PostTodoResponse = test::call_and_read_body_json(&app, req).await;
        assert!(response.id > 0);

        let post = json!({
            "title": "タイトル",
            "body": "文".repeat(MAX_BODY_LENGTH + 1)
        });
        let req = test::TestRequest::post()
            .uri("/todo")
            .set_json(post)
            .to_request();
        let response: ErrorMessage = test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            response.message,
            format!("本文は{}文字以内で入力して下さい。", MAX_BODY_LENGTH)
        );
    }
}
