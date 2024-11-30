use crate::adapters::postgres::namespace_ad as namespace_ad;
use crate::domain::namespace_dom::Namespace;


pub async fn get(namespace: &String) -> Result<Namespace, String> {
    let namespace = namespace_ad::get(namespace);
    match namespace.await {
        Ok(Some(namespace)) => Ok(namespace),
        Ok(None) => Err("404".to_string()),
        Err(exception) => Err(exception.to_string())
    }
}