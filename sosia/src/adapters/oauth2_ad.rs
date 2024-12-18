use actix_web::HttpRequest;
use crate::domain::po::logger_message_type_po::LogType;
use crate::domain::logger_message_dom::LoggerMessage;
use crate::services::logger_srv as Logger;
use crate::domain::date_dom::Date;
use crate::configuration::oauth2_conf;

use serde_json::Value;
use reqwest::Client;
use base64::encode;

pub async fn is_valid_token(request: &HttpRequest) -> bool {
    let token = match request.headers().get("Authorization") {
        Some(token) => {
            match token.to_str() {
                Ok(bearer_token) => {
                    if bearer_token.starts_with("Bearer ") {
                        &bearer_token["Bearer ".len()..bearer_token.len()];
                    } else {
                        return false;
                    }
                },
                Err(_) => return false
            }
        },
        None => return false
    };

    let auth = encode(&format!("{}:{}", oauth2_conf::client_id(), oauth2_conf::client_secret()).to_string());
    match Client::new()
        .post(oauth2_conf::url())
        .header("Authorization", format!("Basic {}", auth))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("token", token)
        ])
        .send()
        .await {
            Ok(response) => {
                match response.json::<Value>().await {
                    Ok(body) => {
                        true == body.get("active")
                            .and_then(|value| value.as_bool())
                            .unwrap_or_else(|| false)
                    },
                    Err(err) => {
                        let error_msg = format!("User authentication failed. Oauth service have returned a non parseable json response: {}", err);
                        Logger::log(&LoggerMessage{
                            log_type: LogType::ERROR,
                            date: Date::new_with_current_time(),
                            message: error_msg.clone()
                        });
                        false
                    }
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
