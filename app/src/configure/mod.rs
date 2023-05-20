use actix_web::web::ServiceConfig;
use chrono::FixedOffset;

use crate::domain::Env;
use crate::http::router;

/// アプリケーション全体の設定
pub fn config(cfg: &mut ServiceConfig) {
    // ルート設定
    router::route(cfg);
}

pub fn env() -> std::io::Result<Env> {
    dotenv::dotenv().ok();
    let DATABASE_URL = match std::env::var("DATABASE_URL") {
        Ok(url) => Env::new(&url),
        Err(e) => {
            panic!();
        }
    };

    Ok(DATABASE_URL)
}

pub fn tz() -> FixedOffset {
    let tz_sec = 3600 * 9;
    let tz = FixedOffset::east_opt(tz_sec).expect("タイムゾーンの取得に失敗しました。");
    tz
}
