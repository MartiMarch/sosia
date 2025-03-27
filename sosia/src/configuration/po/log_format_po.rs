use paperclip::v2::models::{
    DataType,
    DataTypeFormat
};
use std::option::{
    Option,
};
use paperclip::v2::schema::TypedData;
use serde::{
    Serialize,
    Deserialize
};


#[derive(Debug, Serialize, Deserialize)]
pub enum LogFormat {
    JSON,
    TEXT
}

impl TypedData for LogFormat {

    fn data_type() -> DataType {
        DataType::String
    }

    fn format() -> Option<DataTypeFormat> {
        Some(DataTypeFormat::Other)
    }
}
