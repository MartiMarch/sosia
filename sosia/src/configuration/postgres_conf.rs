use crate::domain::{
    logger_message_dom::LoggerMessage as Message,
    date_dom::Date as Date,
    po::logger_message_type_po::LogType as LogType
};
use crate::services::{
    logger_svc as Logger
};
use std::env;


pub fn user() -> String {
    env::var("POSTGRES_USER")
        .unwrap_or("postgres".to_string())
}

pub fn password() -> String {
    env::var("POSTGRES_PASSWORD")
        .unwrap_or_else(|_| {
            let error_msg = "Postgres password must be configured using POSTGRES_PASSWORD environment variable".to_string();
            Logger::log(&Message{
                log_type: LogType::ERROR,
                date: Date::new_with_current_time(),
                message: error_msg.clone()
            });

            panic!("{error_msg}");
        })
}

pub fn host() -> String {
    env::var("POSTGRES_HOST")
        .unwrap_or_else(|_| {
            Logger::log(&Message{
                log_type: LogType::WARNING,
                date: Date::new_with_current_time(),
                message: "Environment variable POSTGRES_HOST undefined, using 127.0.0.1".to_string()
            });

            "127.0.0.1".to_string()
        })
}

pub fn port() -> u16 {
    env::var("POSTGRES_PORT")
        .map(|s| s.parse::<u16>().unwrap())
        .unwrap_or_else(|_| {
            Logger::log(&Message{
                log_type: LogType::WARNING,
                date: Date::new_with_current_time(),
                message: "Environment variable POSTGRES_PORT undefined, using 5432".to_string()
            });

            5432
        })
}

pub fn database() -> String {
    env::var("POSTGRES_DATABASE")
        .unwrap_or_else(|_| {
            Logger::log(&Message{
                log_type: LogType::WARNING,
                date: Date::new_with_current_time(),
                message: "Environment variable POSTGRES_DATABASE undefined, using sosia".to_string()
            });

            "sosia".to_string()
        })
}
