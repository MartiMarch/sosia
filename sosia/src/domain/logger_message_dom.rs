use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use crate::domain::{
    po::logger_message_type_po as log_type,
    date_dom as date,
};


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
struct LoggerMessage {
    message: String,
    date: date::Date,
    log_type: log_type::LogType,
}

