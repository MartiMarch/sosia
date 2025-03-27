use crate::domain::po::logger_message_type_po::LogType as LogType;
use crate::domain::date_dom::Date as Date;
use paperclip::actix::Apiv2Schema;
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct LoggerMessage {
    pub log_type: LogType,
    pub date: Date,
    pub message: String
}

impl LoggerMessage {
    pub fn new(log_type: LogType, date: Date, message: String) -> Self {
        Self {
            log_type,
            date,
            message
        }
    }

    pub fn new_simplified(log_type: LogType, message: String) -> Self {
        let date = Date::new_with_current_time();
        Self {
            log_type,
            date,
            message
        }
    }
}
