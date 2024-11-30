use crate::domain::configuration_dom::ConfigurationDom;
use crate::services::namespace_svc as namespace_srv;
use crate::services::configuration_srv as conf_srv;

use paperclip::actix::api_v2_operation;
use actix_web::HttpResponse;
use actix_web::web::Json;
use actix_web::web::Path;


#[api_v2_operation]
pub async fn get_healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("Alive!")
}

#[api_v2_operation]
pub async fn get_configuration() -> Json<ConfigurationDom> {
    Json(conf_srv::get(Some(true)))
}

#[api_v2_operation]
pub async fn get_namespaces(path_namespace: Path<String>) -> HttpResponse {
    match namespace_srv::get(&path_namespace).await {
        Ok(namespace) => HttpResponse::Ok().json(namespace),
        Err(error) => {
            if error == "404" {
                HttpResponse::NotFound().body(format!("Cant found {path_namespace} namespace"))
            } else {
                HttpResponse::InternalServerError().body(error)
            }
        }
    }
}
