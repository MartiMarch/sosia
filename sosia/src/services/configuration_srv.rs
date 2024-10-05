use crate::domain::{
    configuration_dom::ConfigurationDom
};
use crate::configuration::{
    api_conf as api_conf
};


pub fn get() -> ConfigurationDom {
    ConfigurationDom {
        api_port: api_conf::port(),
        api_network_interface: api_conf::network_interface(),
    }
}
