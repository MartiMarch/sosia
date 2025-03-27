mod configuration;
mod adapters;
mod services;
mod domain;

use paperclip::actix::OpenApiExt;
use actix_web::HttpServer;
use adapters::postgres_ad;
use actix_web::App;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    configuration::postgres_conf::initialize();
    configuration::oauth2_conf::initialize();
    configuration::logger_conf::initialize();
    configuration::timezone_conf::initialize();
    configuration::api_conf::initialize();

    postgres_ad::initialize_database().await;

    HttpServer::new(||
        App::new()
            .wrap_api()
            .configure(configuration::api_conf::routes)
            .with_json_spec_at("/api/v1/swagger")
            .build()
    )
        .workers(configuration::api_conf::workers())
        .bind((configuration::api_conf::network_interface(), configuration::api_conf::port()))?
        .run()
        .await
}

