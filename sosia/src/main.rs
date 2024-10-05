mod adapters;
mod services;
mod domain;
mod configuration;

use actix_web::{
    App,
    HttpServer
};
use configuration::{
    api_conf
};
use paperclip::actix::{
    OpenApiExt
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .wrap_api()
            .configure(api_conf::routes)
            .with_json_spec_at("/api/v1/swagger")
            .build()
    )
        .bind((api_conf::network_interface(), api_conf::port()))?
        .run()
        .await
}

