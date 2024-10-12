use crate::domain::{
    configuration_dom::ConfigurationDom
};
use crate::configuration::{
    api_conf,
    postgres_conf,
    timezone_conf,
    logger_conf
};


pub fn get(is_secured: Option<bool>) -> ConfigurationDom {
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
        logger_format: logger_conf::format()
    };

    if is_secured {
        configuration.postgres_password = "****".to_string();
        configuration.postgres_user = "****".to_string();
    }
    configuration
}
