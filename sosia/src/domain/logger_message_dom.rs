use paperclip::actix::Apiv2Schema;
use serde::{
    Deserialize,
    Serialize
};
use crate::domain::{
    po::logger_message_type_po::LogType as LogType,
    date_dom::Date as Date
};


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct LoggerMessage {
    pub log_type: LogType,
    pub date: Date,
    pub message: String
}
