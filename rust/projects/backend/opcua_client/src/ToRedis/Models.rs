use redis::{Commands, ConnectionLike, RedisError, RedisResult};
use std::sync::{self,Arc};
use chrono::prelude::*;
pub struct Temperature {
    pub val: f64,
    pub time: NaiveDateTime,
}

pub struct RedisData{
    pub url: String, 
    pub passwd: String,
    pub conn: sync::RwLock<Option<redis::Connection>>,
}
impl RedisData{
    fn gain_conn(&self) ->RedisResult<()>{
        let client = redis::Client::open(self.url.as_ref())?;
        let mut current_conn = self.conn.write().unwrap();
        let mut new_conn = client.get_connection()?;
        redis::cmd("Auth").arg(&self.passwd).query::<()>(&mut new_conn)?;
        *current_conn = Some(new_conn);
        Ok(())
    }
    pub fn set(&self, key :&str,data:String){
        let mut conn= self.conn.write().unwrap();
        if let Err(err) = conn.as_mut().unwrap().set::<_,_,()>(key, data) {
            println!("Closed for {}",err);
            drop(conn);
            if let Err(error) = self.gain_conn() {
                println!("Gain connection failed for {}",error);
            }
        };
    }
    pub fn rpush(&self,key: &str,data:String){
        let mut conn= self.conn.write().unwrap();
        if let Err(err) = conn.as_mut().unwrap().rpush::<String,String,()>(key.to_string(), data){
            println!("Closed for {}",err);
            drop(conn);
            if let Err(error) = self.gain_conn() {
                println!("Gain connection failed for {}",error);
            }
        };
    }
    pub fn new(url: String,passwd: String) -> RedisData {
        let res = RedisData{url,passwd,conn:sync::RwLock::new(None)};
        res.gain_conn().unwrap();
        return res;
    }
}