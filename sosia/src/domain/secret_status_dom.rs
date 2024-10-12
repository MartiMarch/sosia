use paperclip::actix::Apiv2Schema;
use serde::{
    Deserialize,
    Serialize
};


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct SecretStatus {
    pub name: String
}
