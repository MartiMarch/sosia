use crate::adapters::oauth2_ad;

use actix_web::HttpRequest;


pub async fn validate(request: &HttpRequest) -> bool {
    match request.headers().get("Authorization") {
        Some(token) => {
            match token.to_str() {
                Ok(bearer_token) => {
                    if bearer_token.starts_with("Bearer ") {
                        let token = &bearer_token["Bearer ".len()..bearer_token.len()];
                        println!("{token}");
                        oauth2_ad::is_valid_token(&bearer_token.to_string()).await
                    } else {
                        false
                    }
                },
                Err(_) => {
                    false
                },
            }
        },
        None => false
    }
}
