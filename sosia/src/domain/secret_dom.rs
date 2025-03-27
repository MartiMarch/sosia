use paperclip::actix::Apiv2Schema;
use serde::{
    Deserialize,
    Serialize
};
use crate::domain::{
    secret_status_dom as status_dom,
    date_dom,
};


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Secret {
    pub id: String,
    pub value: String,
    pub expiration_date: date_dom::Date,
    pub creation_date: date_dom::Date,
    pub update_date: date_dom::Date,
    pub source: Option<String>,
    pub status: status_dom::SecretStatus,
}
