use chrono::prelude::*;
use serde::{Deserialize,Serialize};
use std::convert::From;

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct TempRecord{
    pub val: f64,
    pub id: u32,
    pub time: String,
}

impl TempRecord{
    pub fn new(val:f64,id:u32,time:String) -> TempRecord {
        TempRecord {
            val,
            id,
            time,
        }
    }
}
impl From<(f64,u32,String)> for TempRecord {
    fn from(value: (f64,u32,String)) -> TempRecord {
         TempRecord {
            val: value.0,
            id:value.1, 
            time:value.2, 
        }
    }
}

impl From<(String,u32,String)> for TempRecord {
    fn from(value: (String,u32,String)) -> TempRecord {
         TempRecord {
            val: value.0.parse::<f64>().unwrap_or(0.0),
            id:value.1, 
            time:value.2, 
        }
    }
}

impl From<(String,u32)> for TempRecord {
    fn from(value: (String,u32)) -> TempRecord {
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

