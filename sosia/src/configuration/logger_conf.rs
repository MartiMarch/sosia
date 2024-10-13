use crate::configuration::{
    po::log_format_po as log_format
};
use crate::domain::{
    logger_message_dom::LoggerMessage as Message,
    date_dom::Date as Date,
    po::logger_message_type_po::LogType as LogType
};
use crate::services::{
    logger_svc as Logger
};
use std::env;


pub fn format() -> log_format::LogFormat {
    let log_format: String = env::var("LOG_FORMAT")
        .unwrap_or_else(|_| {
            Logger::log(&Message{
                log_type: LogType::WARNING,
                date: Date::new_with_current_time(),
                message: "Environment variable LOG_FORMAT undefined, using JSON log format".to_string()
            });
            "json".to_string()
        });

    match log_format.to_lowercase().as_str() {
        "json" => log_format::LogFormat::JSON,
        "text" => log_format::LogFormat::TEXT,
        _ => log_format::LogFormat::JSON
    }
}
