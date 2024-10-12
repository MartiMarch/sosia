use paperclip::actix::{
    api_v2_operation
};
use actix_web::{
    HttpResponse,
    web::Json
};
use crate::services::{
    configuration_srv as conf_srv
};
use crate::domain::{
    configuration_dom::ConfigurationDom
};


#[api_v2_operation]
pub async fn get_healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("Alive!")
}

#[api_v2_operation]
pub async fn get_configuration() -> Json<ConfigurationDom> {
    Json(conf_srv::get(Some(true)))
}
