use crate::adapters::user_ad;

use actix_web::HttpRequest;


pub async fn validate(request: &HttpRequest) {
    let is_valid_token: bool = user_ad::validate(request).await;

    if is_valid_token == true {
        println!("Good token")
    } else {
        println!("Bad token")
    }
}
