use crate::domain::configuration_dom::ConfigurationDom;
use crate::configuration::postgres_conf;
use crate::configuration::timezone_conf;
use crate::configuration::logger_conf;
use crate::configuration::oauth2_conf;
use crate::configuration::api_conf;
use crate::services::user_srv;

use actix_web::HttpRequest;


pub async fn get(request: &HttpRequest, is_secured: Option<bool>) -> ConfigurationDom {
    user_srv::validate(&request).await;

    let is_secured = is_secured.unwrap_or(false);
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

    if is_secured {
        configuration.postgres_user = "****".to_string();
        configuration.postgres_password = "****".to_string();
        configuration.oauth2_client_id = "****".to_string();
        configuration.oauth2_client_secret = "****".to_string();
    }
    configuration
}
