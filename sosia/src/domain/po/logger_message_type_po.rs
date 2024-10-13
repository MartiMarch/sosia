use serde::{
    Deserialize,
    Serialize
};
use std::fmt;
use colored::Colorize;
use paperclip::actix::Apiv2Schema;


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub enum LogType {
    INFO,
    ERROR,
    WARNING,
    DEBUG,
    CORE,
}

impl LogType {
    pub fn to_custom_string(&self) -> String {
        match self {
            LogType::INFO => "INFO".to_string(),
            LogType::ERROR => "ERROR".to_string(),
            LogType::WARNING => "WARNING".to_string(),
            LogType::DEBUG => "DEBUG".to_string(),
            LogType::CORE => "CORE".to_string(),
        }
    }
}

impl fmt::Display for LogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if atty::is(atty::Stream::Stdout){
            match self {
                LogType::INFO => write!(f, "INFO"),
                LogType::ERROR => write!(f, "ERROR"),
                LogType::WARNING => write!(f, "WARNING"),
                LogType::DEBUG => write!(f, "DEBUG"),
                LogType::CORE => write!(f, "CORE"),
            }
        } else {
            match self {
                LogType::INFO => write!(f, "{}", "INFO".blue()),
                LogType::ERROR => write!(f, "{}", "ERROR".red()),
                LogType::WARNING => write!(f, "{}", "WARNING".yellow()),
                LogType::DEBUG => write!(f, "{}", "DEBUG".purple()),
                LogType::CORE => write!(f, "{}", "CORE".green()),
            }
        }
    }
}
