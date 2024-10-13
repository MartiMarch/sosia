use paperclip::actix::Apiv2Schema;
use serde::{
    Deserialize,
    Serialize
};
use crate::configuration::{
    po::log_format_po as log_format,
};


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct ConfigurationDom {
    pub api_port: u16,
    pub api_network_interface: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_host: String,
    pub postgres_port: u16,
    pub postgres_database: String,
    pub timezone: String,
    pub logger_format: log_format::LogFormat
}
