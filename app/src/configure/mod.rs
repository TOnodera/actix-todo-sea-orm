use actix_web::web::ServiceConfig;

use crate::types::Env;

mod router;

/// アプリケーション全体の設定
pub fn config(cfg:&mut ServiceConfig) {
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