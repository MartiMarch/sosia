use crate::domain::po::logger_message_type_po::LogType;
use crate::domain::logger_message_dom::LoggerMessage;
use crate::services::logger_srv as Logger;
use crate::domain::date_dom::Date;
use std::env;

pub fn client_id() -> String {
    env::var("OAUTH2_CLIENT_ID")
        .unwrap_or_else(|_| {
            let error_msg = "OAuth 2 client_id must be configured using OAUTH2_CLIENT_ID environment variable".to_string();
            Logger::log(&LoggerMessage{
                log_type: LogType::ERROR,
                date: Date::new_with_current_time(),
                message: error_msg.clone()
            });

            panic!(error_msg)
        })
}

pub fn client_secret() -> String {
    env::var("OAUTH2_CLIENT_SECRET")
        .unwrap_or_else(|_| {
            let error_msg = "OAuth 2 client_id must be configured using OAUTH2_CLIENT_SECRET environment variable".to_string();
            Logger::log(&LoggerMessage{
                log_type: LogType::ERROR,
                date: Date::new_with_current_time(),
                message: error_msg.clone()
            });

            panic!(error_msg)
        })
}
