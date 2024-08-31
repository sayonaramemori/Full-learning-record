use chrono::prelude::*;
use serde::{Deserialize,Serialize};
use std::convert::From;
use super::record::Record;



#[derive(Deserialize,Serialize,Debug)]
pub struct DateTimeRng(pub NaiveDateTime,pub NaiveDateTime);
#[derive(Deserialize,Serialize,Debug)]
pub struct DateTimeRange {
    pub start: String, // or chrono::NaiveDateTime if you want to parse dates
    pub end: String,   // or chrono::NaiveDateTime
}

#[derive(Deserialize,Serialize,Default,Clone,sqlx::FromRow,Debug)]
pub struct TempRecord<T>
{
    pub val: f64,
    pub id: i64,
    pub time: T,
}

impl<T> TempRecord<T>{
    pub fn new(val:f64,id:i64,time:T) -> TempRecord<T> {
        TempRecord {
            val,
            id,
            time,
        }
    }
}

impl From<(Record,i64)> for TempRecord<String> {
    fn from(value: (Record,i64)) -> TempRecord<String> {
        let (record,id) = value;
        TempRecord {
            val: record.v.parse::<f64>().unwrap_or(0.0),
            id,
            //only debug string works for frontend
            time: format!("{:?}",record.t),
        }
    }
}

#[derive(Deserialize,Serialize,Default,Debug)]
pub struct HistoryData<T>
{
    pub average: f64,
    pub total_time: f64,
    pub records: Vec<TempRecord<T>>,
}