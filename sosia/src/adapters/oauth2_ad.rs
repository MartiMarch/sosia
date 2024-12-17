use crate::configuration::oauth2_conf;

use crate::domain::po::logger_message_type_po::LogType;
use crate::domain::logger_message_dom::LoggerMessage;
use crate::services::logger_srv as Logger;
use crate::domain::date_dom::Date;
use serde_json::Value;
use reqwest::Client;
use base64::encode;


pub async fn is_valid_token(token: &String) -> bool {
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
                        true
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


                false
            }
        }
}
