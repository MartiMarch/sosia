use paperclip::actix::Apiv2Schema;
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Namespace {
    pub name: String
}
