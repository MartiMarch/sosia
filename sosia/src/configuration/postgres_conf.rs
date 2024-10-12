use std::env;


pub fn user() -> String {
    env::var("POSTGRES_USER")
        .unwrap_or("postgres".to_string())
}

pub fn password() -> String {
    env::var("POSTGRES_PASSWORD")
        .unwrap_or("password".to_string())
}

pub fn host() -> String {
    env::var("POSTGRES_HOST")
        .unwrap_or("127.0.0.1".to_string())
}

pub fn port() -> u16 {
    env::var("POSTGRES_PORT")
        .map(|s| s.parse::<u16>().unwrap())
        .unwrap_or(5432)
}

pub fn database() -> String {
    env::var("POSTGRES_DATABASE")
        .unwrap_or("sosia".to_string())
}
