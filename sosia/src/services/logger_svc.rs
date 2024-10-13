use std::collections::BTreeMap;
use crate::adapters::{
    logger_ad::LOGGER as Logger
};
use crate::configuration::{
    po::log_format_po::LogFormat as LogFormat,
};
use crate::domain::{
    logger_message_dom::LoggerMessage
};


/*
 * TODO(marti): use log type to attach message to events
 * TODO(marti): configure log level using configuration adapter
 */
pub fn log(logger_message: &LoggerMessage) {
    match Logger.format() {
        LogFormat::TEXT => {
            let serialized_message: String = format!(
                "[{}][{}] {}",
                logger_message.log_type, logger_message.date, logger_message.message
            );
            Logger.log(&serialized_message)
        }
        LogFormat::JSON => {
            let mut serialized_message: BTreeMap<String, _> = BTreeMap::new();
            serialized_message.insert("log_type".to_string(), format!("{}", logger_message.log_type.to_custom_string()));
            serialized_message.insert("date".to_string(), format!("{}", logger_message.date));
            serialized_message.insert("message".to_string(), format!("{}", logger_message.message));

            println!("{}", serde_json::to_string(&serialized_message).unwrap())
        }
    }
}
