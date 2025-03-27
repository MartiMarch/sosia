use crate::domain::configuration_dom::ConfigurationDom;
use crate::configuration::postgres_conf;
use crate::configuration::timezone_conf;
use crate::configuration::logger_conf;
use crate::configuration::oauth2_conf;
use crate::configuration::api_conf;
use crate::adapters::oauth2_ad;

use crate::domain::po::logger_message_type_po::LogType;
use crate::domain::logger_message_dom::LoggerMessage;

use actix_web::{HttpRequest, HttpResponse};


pub async fn get(request: &HttpRequest, is_secured: Option<bool>) -> HttpResponse {
    if oauth2_ad::is_valid_token(&request).await == false {
        let custom_http_error = LoggerMessage::new_simplified(
            LogType::ERROR,
            "Unauthorized by Oauth2".to_string()
        );
        return HttpResponse::Unauthorized().json(custom_http_error)
    }

    let mut configuration = ConfigurationDom {
        api_port: api_conf::port(),
        api_network_interface: api_conf::network_interface(),
        postgres_user: postgres_conf::user(),
        postgres_password: postgres_conf::password(),
        postgres_host: postgres_conf::host(),
        postgres_port: postgres_conf::port(),
        postgres_database: postgres_conf::database(),
        timezone: timezone_conf::get(),
        logger_format: logger_conf::format(),
        oauth2_client_id: oauth2_conf::client_id(),
        oauth2_client_secret: oauth2_conf::client_secret(),
        oauth2_url: oauth2_conf::url().to_string()
    };

    if is_secured.unwrap_or(false) {
        configuration.postgres_user = "****".to_string();
        configuration.postgres_password = "****".to_string();
        configuration.oauth2_client_id = "****".to_string();
        configuration.oauth2_client_secret = "****".to_string();
    }

    HttpResponse::Ok().json(configuration)
}
