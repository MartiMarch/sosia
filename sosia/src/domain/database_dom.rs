use paperclip::actix::Apiv2Schema;
use serde::{
    Deserialize,
    Serialize
};


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
struct Database {
    name: String,
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}
