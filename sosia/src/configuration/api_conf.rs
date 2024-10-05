use crate::adapters::{
    api_ad
};
use paperclip::actix::{
    web
};
use std::{
    env
};

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
        );
}

pub fn port() -> u16 {
    env::var("API_PORT")
        .map(|s| s.parse::<u16>().unwrap_or(8000))
        .unwrap_or(8080)
}

pub fn network_interface() -> String {
    env::var("API_NETWORK_INTERFACE")
        .unwrap_or("127.0.0.1".to_string())
}
