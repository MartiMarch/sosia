use paperclip::actix::Apiv2Schema;
use chrono_tz::Tz;
use serde::{
    Deserialize,
    Serialize
};
use crate::configuration::{
    timezone_conf as TimezoneConf
};
use std::fmt;
use chrono::{
    Datelike,
    NaiveDateTime,
    ParseResult,
    TimeZone,
    Timelike,
    Utc
};


#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    utc_offset: String
}

impl Date {

    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            utc_offset: TimezoneConf::get()
        }
    }

    pub fn new_with_current_time() -> Self {
        let current_time = Utc::now();

        Self {
            year: current_time.year(),
            month: current_time.month(),
            day: current_time.day(),
            hour: current_time.hour(),
            minute: current_time.minute(),
            second: current_time.second(),
            utc_offset: TimezoneConf::get()
        }
    }

    pub fn to_naive(&self) -> ParseResult<NaiveDateTime> {
        let date_format = "%Y-%m-%d %H:%M:%S";
        let date_as_string = format!("{}-{}-{} {}:{}:{}",
             self.year, self.month, self.day,
             self.hour, self.minute, self.second
        );

        NaiveDateTime::parse_from_str(&date_as_string, date_format)
    }

    pub fn to_utc(&self) -> Result<chrono::DateTime<Tz>, chrono::ParseError> {
        let naive_date: NaiveDateTime = self.to_naive()?;
        let timezone: Tz = self.utc_offset.parse().unwrap();

        Ok(timezone.from_utc_datetime(&naive_date))
    }
}

impl fmt::Display for Date {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let utc_date = self.to_utc().unwrap();
        write!(f, "{}", utc_date.to_string())
    }
}
