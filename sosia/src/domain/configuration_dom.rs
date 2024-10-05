use paperclip::actix::Apiv2Schema;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct ConfigurationDom {
    pub api_port: u16,
    pub api_network_interface: String,
}
