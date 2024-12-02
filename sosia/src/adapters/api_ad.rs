use crate::domain::configuration_dom::ConfigurationDom;
use crate::services::namespace_srv as namespace_srv;
use crate::services::configuration_srv as conf_srv;
use crate::domain::namespace_dom::Namespace;

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
pub async fn get_namespace(path_namespace: Path<String>) -> HttpResponse {
    let namespace_name: String = path_namespace.into_inner();
    match namespace_srv::get(&namespace_name).await {
        Ok(namespace) => HttpResponse::Ok().json(namespace),
        Err(error) => {
            if error == "404" {
                HttpResponse::NotFound().body(format!("Cant found {namespace_name} namespace"))
            } else {
                HttpResponse::InternalServerError().body(error)
            }
        }
    }
}

#[api_v2_operation]
pub async fn post_namespace(namespace: Json<Namespace>) -> HttpResponse {
    match namespace_srv::post(&namespace.into_inner()).await {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(error) => {
            if error.contains("already exists") {
                HttpResponse::Conflict().body(error)
            } else {
                HttpResponse::InternalServerError().body(error)
            }
        }
    }
}
