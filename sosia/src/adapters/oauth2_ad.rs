use crate::domain::po::logger_message_type_po::LogType;
use crate::domain::logger_message_dom::LoggerMessage;
use crate::services::logger_srv as Logger;
use crate::configuration::oauth2_conf;
use crate::domain::date_dom::Date;

use actix_web::HttpRequest;
use serde_json::Value;
use reqwest::Client;
use reqwest::Result;
use base64::encode;


pub async fn is_valid_token(request: &HttpRequest) -> bool {
    let authorization_header = match request.headers().get("Authorization") {
        Some(v) => v.to_str().unwrap(),
        None => return false
    };

    let bearer_token: String = authorization_header.to_string();
    if !bearer_token.starts_with("Bearer ") {
        return false;
    }

    let filtered_token: &str = &bearer_token["Bearer ".len()..bearer_token.len()];
    let auth: String = encode(&format!("{}:{}", oauth2_conf::client_id(), oauth2_conf::client_secret()).to_string());

    match Client::new()
        .post(oauth2_conf::url())
        .header("Authorization", format!("Basic {}", auth))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("token", filtered_token)
        ])
        .send()
        .await {
            Ok(response) => {
                let parsed_json: Result<Value> = response.json::<Value>().await;
                if let Err(err) = parsed_json {
                    let error_msg = format!("User authentication failed. Oauth service have returned a non parseable json response: {}", err);
                    Logger::log(&LoggerMessage{
                        log_type: LogType::ERROR,
                        date: Date::new_with_current_time(),
                        message: error_msg.clone()
                    });

                    false
                } else {
                    parsed_json
                        .unwrap()
                        .get("active")
                        .and_then(|value| value.as_bool())
                        .unwrap_or_else(|| false)
                }
            },
            Err(err) => {
                let error_message = format!("Something goes wrong calling OAuth service: {}", err);
                Logger::log(&LoggerMessage{
                    log_type: LogType::ERROR,
                    date: Date::new_with_current_time(),
                    message: error_message.clone()
                });

                false
            }
    }

}
