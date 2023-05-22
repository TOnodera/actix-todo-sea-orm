use std::sync::Mutex;

use once_cell::sync::Lazy;
use simple_log::{debug, info, log::error};

pub mod logger;

#[derive(Clone)]
pub struct Logger;
impl Logger {
    pub fn new() -> Self {
        let config = logger::get_config().expect("ログ設定の読み込みに失敗しました。");
        simple_log::new(config).expect("ロガーの初期化に失敗しました。");
        Self
    }
    pub fn info(&self, message: &str) {
        info!("{}", message);
    }
    pub fn error(&self, message: &str) {
        error!("{}", message);
    }
    pub fn debug(&self, message: &str) {
        debug!("{}", message);
    }
}

static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new()));

pub fn info(message: &str) {
    LOGGER.lock().expect("ログ出力に失敗しました。").info(message);
}
pub fn error(message: &str) {
    LOGGER.lock().expect("ログ出力に失敗しました。").error(message);
}
pub fn debug(message: &str) {
    LOGGER.lock().expect("ログ出力に失敗しました。").debug(message);
}
