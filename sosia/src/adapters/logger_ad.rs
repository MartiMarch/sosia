use crate::configuration::po::log_format_po as log_format;
use crate::configuration::logger_conf;
use once_cell::sync::Lazy;
use std::fmt::Debug;


pub static LOGGER: Lazy<Logger> = Lazy::new(|| {
    Logger::new()
});


pub struct Logger {
    log_format: log_format::LogFormat
}

impl Logger {

    fn new() -> Self {
        Logger {
            log_format: logger_conf::format()
        }
    }

    pub fn log<T: Debug + serde::Serialize>(&self, message: &T) {
        match self.log_format {
            log_format::LogFormat::TEXT => {
                println!("{:?}", message);
            }
            log_format::LogFormat::JSON => {
                let message_as_json = serde_json::to_string(&message)
                    .unwrap();
                println!("{message_as_json}");
            }
        }
    }

    pub fn format(&self) -> &log_format::LogFormat {
        &self.log_format
    }
}
