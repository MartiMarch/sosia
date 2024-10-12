use once_cell::sync::Lazy;
use std::fmt::Debug;
use crate::configuration::{
    po::log_format_po as log_format,
    logger_conf
};


static LOGGER: Lazy<Logger> = Lazy::new(|| {
    Logger::new()
});


struct Logger {
    log_format: log_format
}

impl Logger {

    pub fn new() -> Self {
        Logger {
            log_format: logger_conf::format()
        }
    }

    pub fn log<T: Debug>(&self, message: &T) {
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
}
