use actix_web::web::ServiceConfig;

mod router;

/// アプリケーション全体の設定
pub fn config(cfg:&mut ServiceConfig) {
    // ルート設定
    router::route(cfg);
}