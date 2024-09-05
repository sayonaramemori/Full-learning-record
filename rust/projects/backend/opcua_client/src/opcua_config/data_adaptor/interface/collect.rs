use chrono::prelude::*;
pub trait StoreValueTime {
    fn new() -> Self;
    fn set_value(&mut self,val: String)->&mut Self;
    fn set_time(&mut self,time: DateTime<Utc>)->&mut Self;
    fn get_value(&self)->String;
    fn get_time(&self)->DateTime<Utc>;
}