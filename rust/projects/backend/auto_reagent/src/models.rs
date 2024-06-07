use chrono::prelude::*;
use chrono::Duration;
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct TempRecord {
    pub val: f64,
    pub id: u32,
    pub time: NaiveDateTime,
}

impl TempRecord {
    pub fn new(val:f64,id:u32,time:NaiveDateTime) -> TempRecord {
        TempRecord {
            val,
            id,
            time,
        }
    }
}

pub fn gen_time(start: NaiveDateTime, interval: Duration, count: u32) -> Vec<NaiveDateTime> {
    (0..count)
        .map(|i| start - interval * i as i32)
        .collect()
}

