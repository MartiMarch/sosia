use crate::domain::po::logger_message_type_po::LogType;
use crate::domain::logger_message_dom::LoggerMessage;
use crate::domain::namespace_dom::Namespace;
use crate::adapters::postgres::namespace_ad;
use crate::services::logger_srv as Logger;

use actix_web::HttpRequest;


pub async fn get(request: &HttpRequest, namespace_name: &String) -> Result<Namespace, String> {

    let namespace = namespace_ad::get(namespace_name);
    match namespace.await {
        Ok(Some(namespace)) => Ok(namespace),
        Ok(None) => {
            let message: LoggerMessage = Logger::str_to_log(&format!("404, namespace {namespace_name} not found"), &LogType::ERROR);
            Err(Logger::log_to_str(&message))
        },
        Err(exception) => Err(exception.to_string())
    }
}

pub async fn post(request: &HttpRequest, namespace: &Namespace) -> Result<String, String> {
    match namespace_ad::post(namespace).await {
        Ok(()) => {
            let message: LoggerMessage = Logger::str_to_log(
                &format!("Namespace {} created", namespace.name),
                &LogType::INFO
            );
            Ok(Logger::log_to_str(&message))
        },
        Err(err) => {
            if err.to_string().contains("unique_violation") {
                let message: LoggerMessage = Logger::str_to_log(
                    &format!("Namespace {} can't be created because it already exists", namespace.name),
                    &LogType::ERROR,
                );
                Err(Logger::log_to_str(&message))
            } else {
                let message: LoggerMessage = Logger::str_to_log(&err.to_string(), &LogType::ERROR);
                Err(Logger::log_to_str(&message))
            }
        }
    }
}
