use simple_log::LogConfig;
use std::{fs::File, io::Read};

use crate::types::{ApplicationError, Result};

fn read_config_file() -> Result<String> {
    let mut file = File::open("./src/logger/config.json").map_err(|_| {
        ApplicationError::IoError(String::from("ログ設定ファイルの読み込みに失敗しました。"))
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| {
        ApplicationError::IoError(String::from("ログ設定ファイルのロードに失敗しました。"))
    })?;

    Ok(contents)
}

fn parse_config_file(config_file: &str) -> Result<LogConfig> {
    let config: LogConfig = serde_json::from_str(config_file).map_err(|_| {
        ApplicationError::UnknownError(String::from("コンフィグファイルの形式が不正です。"))
    })?;
    Ok(config)
}

pub fn get_config() -> Result<LogConfig> {
    let config_string = read_config_file()?;
    let config = parse_config_file(&config_string)?;
    Ok(config)
}
