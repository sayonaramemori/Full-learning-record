use chrono::prelude::*;
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct TempRecord<T>{
    pub val: T,
    pub id: u32,
    pub time: NaiveDateTime,
}

impl<T> TempRecord<T>{
    pub fn new(val:T,id:u32,time:NaiveDateTime) -> TempRecord<T> {
        TempRecord {
            val,
            id,
            time,
        }
    }
}



