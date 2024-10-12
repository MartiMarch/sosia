use paperclip::v2::models::{
    DataType,
    DataTypeFormat
};
use paperclip::v2::schema::TypedData;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Debug, Serialize, Deserialize)]
pub enum LogType {
    INFO,
    WARNING,
    DEBUG,
    ERROR,
    CORE,
}

impl TypedData for LogType {

    fn data_type() -> DataType {
        DataType::String
    }

    fn format() -> Option<DataTypeFormat> {
        Some(DataTypeFormat::Other)
    }
}
