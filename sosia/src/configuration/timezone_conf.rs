use chrono_tz::Tz;
use std::env;


pub fn get() -> String {
    let timezone = env::var("TIMEZONE")
        .unwrap_or_else(|_| "UTC".to_string());

    match timezone.parse::<Tz>() {
        Ok(tz) => tz.to_string(),
        Err(error) => {
            eprintln!("Something goes wrong configuring timezone. Error: {error}");
            "UTC".to_string()
        }
    }
}
