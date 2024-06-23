use chrono::prelude::*;
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



