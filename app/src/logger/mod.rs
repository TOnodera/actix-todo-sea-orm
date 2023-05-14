use simple_log::{info, log::error, debug};

pub mod logger;

pub struct Logger;
impl Logger {
    pub fn new() -> Self {
        let config = logger::get_config().expect("ログ設定の読み込みに失敗しました。");
        simple_log::new(config);
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

pub fn log() -> Logger {
    Logger::new()
}