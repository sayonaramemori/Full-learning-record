use std::convert::From;
use chrono::prelude::NaiveDateTime;

use crate::opcua_config::opcua_session_wrapper::DataTime;

#[derive(Default,Debug)]
pub struct Temperature {
    pub val: f64,
    pub time: NaiveDateTime,
}

impl From<DataTime> for Temperature {
    fn from(value: DataTime) -> Self {
        Temperature {
            val: value.data.parse::<f64>().unwrap_or(0.0),
            time: value.time.naive_local(),
        }
    }
}
