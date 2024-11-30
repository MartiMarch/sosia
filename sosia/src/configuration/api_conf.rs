use crate::domain::po::logger_message_type_po::LogType as LogType;
use crate::domain::logger_message_dom::LoggerMessage as Message;
use crate::domain::date_dom::Date as Date;
use crate::services::logger_svc as Logger;
use crate::adapters::api_ad;

use paperclip::actix::web;
use std::env;


pub fn routes(configuration: &mut web::ServiceConfig) {
    configuration
        .service(
            web::resource("/api/v1/healthcheck").route(
                web::get().to(api_ad::get_healthcheck)
            )
        )
        .service(
            web::resource("/api/v1/configuration").route(
                web::get().to(api_ad::get_configuration)
            )
        )
        .service(
            web::resource("/api/v1/namespace").route(
                web::get().to(api_ad::get_namespaces)
            )
        );
}

pub fn port() -> u16 {
    env::var("API_PORT")
        .map(|s| s.parse::<u16>().unwrap_or(8000))
        .unwrap_or_else(|_| {
            Logger::log(&Message{
                log_type: LogType::WARNING,
                date: Date::new_with_current_time(),
                message: "Environment variable API_PORT undefined, using 8080".to_string(),
            });
            8080
        })
}

pub fn network_interface() -> String {
    env::var("API_NETWORK_INTERFACE")
        .unwrap_or_else(|_| {
            Logger::log(&Message{
                log_type: LogType::WARNING,
                date: Date::new_with_current_time(),
                message: "Environment variable API_NETWORK_INTERFACE undefined, using 127.0.0.1".to_string(),
            });
            "127.0.0.1".to_string()
        })
}

pub fn workers() -> usize {
    match env::var("API_WORKERS") {
        Ok(value) => {
            value.parse::<usize>()
                .unwrap_or_else(|_| {
                    Logger::log(&Message{
                        log_type: LogType::WARNING,
                        date: Date::new_with_current_time(),
                        message: "Bad API_WORKERS value, using 3 workers".to_string(),
                    });
                    3
                })
        },
        Err(_) => {
            Logger::log(&Message{
                log_type: LogType::WARNING,
                date: Date::new_with_current_time(),
                message: "Environment variable API_WORKERS undefined, using 3 workers".to_string(),
            });
            3
        }
    }
}
