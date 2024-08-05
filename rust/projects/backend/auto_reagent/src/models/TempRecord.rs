use chrono::prelude::*;
use serde::{Deserialize,Serialize};
use std::convert::From;
use std::ops::Add;


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
impl From<(f64,i64,String)> for TempRecord<String> {
    fn from(value: (f64,i64,String)) -> TempRecord<String> {
         TempRecord {
            val: value.0,
            id:value.1, 
            time:value.2, 
        }
    }
}

impl From<(String,i64,String)> for TempRecord<String> {
    fn from(value: (String,i64,String)) -> TempRecord<String> {
         TempRecord {
            val: value.0.parse::<f64>().unwrap_or(0.0),
            id:value.1, 
            time:value.2, 
        }
    }
}

impl From<(String,i64)> for TempRecord<String> {
    fn from(value: (String,i64)) -> TempRecord<String> {
        let mut iterator = value.0.split('|');
        let val = match iterator.next() {
            Some(s) => s.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        };
        let time = match iterator.next() {
            Some(s) => s.to_string(),
            _ => String::from(""),
        };
        TempRecord {
            val,
            time,
            id:value.1, 
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