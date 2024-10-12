use crate::configuration::{
    po::log_format_po as log_format
};
use std::env;


pub fn format() -> log_format::LogFormat {
    let log_format: String = env::var("LOG_FORMAT")
        .unwrap_or("json".to_string());

    match log_format.to_lowercase().as_str() {
        "json" => log_format::LogFormat::JSON,
        "text" => log_format::LogFormat::TEXT,
        _ => log_format::LogFormat::JSON
    }
}
