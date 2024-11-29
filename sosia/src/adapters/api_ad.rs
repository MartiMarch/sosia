use crate::domain::configuration_dom::ConfigurationDom;
use crate::services::configuration_srv as conf_srv;
use paperclip::actix::api_v2_operation;
use actix_web::HttpResponse;
use actix_web::web::Json;


#[api_v2_operation]
pub async fn get_healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("Alive!")
}

#[api_v2_operation]
pub async fn get_configuration() -> Json<ConfigurationDom> {
    Json(conf_srv::get(Some(true)))
}
