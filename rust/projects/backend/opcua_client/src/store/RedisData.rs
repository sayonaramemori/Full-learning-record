use redis::{Commands, ConnectionLike, RedisError, RedisResult};
use std::{fmt::Display, sync::{self,Arc}};
use crate::debug_println;

pub struct RedisData{
    url: String, 
    passwd: String,
    conn: sync::RwLock<Option<redis::Connection>>,
    client: redis::Client,
}
impl RedisData{
    pub fn new(url: &str,passwd: &str) -> RedisData {
        let client = redis::Client::open(url).unwrap();
        let res = RedisData{
            url: url.to_string(),
            passwd: passwd.to_string(),
            conn: sync::RwLock::new(None),
            client,
        };
        let _ = res.gain_conn();
        return res;
    }
    fn gain_conn(&self) ->RedisResult<()>{
        let mut current_conn = self.conn.write().unwrap();
        let mut new_conn = self.client.get_connection()?;
        redis::cmd("Auth").arg(&self.passwd).query::<()>(&mut new_conn)?;
        *current_conn = Some(new_conn);
        Ok(())
    }
    pub fn llen(&self, key :&str) -> RedisResult<i32>{
        let mut conn= self.conn.write().unwrap();
        match conn.as_mut().unwrap().llen::<_,i32>(key.to_string()){
           Ok(num) => return Ok(num),
           Err(err) => {
                debug_println!("Get {key} length failed for {}",err);
                drop(conn);
                let _ = self.gain_conn();
                return Err(err);
           },
        }
    }
    pub fn setex(&self, key :&str,data:String,sec:u64)->bool{
        let mut conn= self.conn.write().unwrap();
        if let Err(err) = conn.as_mut().unwrap().set_ex::<_,_,()>(key, data,sec) {
            debug_println!("setex {key} failed for {}",err);
            drop(conn);
            let _ = self.gain_conn();
            return false;
        };
        true
    }
    pub fn setex_retry<T:Display>(&self, key :&str,data:T,sec:u64,times:u32){
        for _ in 0..times {
            if self.setex(key, data.to_string().clone(), sec) { return (); }
        }
    }
    pub fn set(&self, key :&str,data:String)->bool{
        let mut conn= self.conn.write().unwrap();
        if let Err(err) = conn.as_mut().unwrap().set::<_,_,()>(key, data) {
            debug_println!("set {key} failed for {}",err);
            drop(conn);
            let _ = self.gain_conn();
            return false;
        };
        true
    }
    pub fn rpush_retry(&self,key: &str,data:String,times:u32){
        for _ in 0..times {
            if self.rpush(key, data.clone()) { return (); }
        }
    }
    pub fn rpush(&self,key: &str,data:String)-> bool{
        let mut conn= self.conn.write().unwrap();
        if let Err(err) = conn.as_mut().unwrap().rpush::<String,String,()>(key.to_string(), data){
            debug_println!("rpush {key} failed for {}",err);
            drop(conn);
            let _ = self.gain_conn();
            return false;
        };
        return true;
    }
    pub fn lpop(&self,key: &str,count: usize)->RedisResult<Vec<String>>{
        let mut conn= self.conn.write().unwrap();
        match  conn.as_mut().unwrap().lpop::<_,Vec<String>>(key.to_string(), core::num::NonZeroUsize::new(count)){
           Ok(res) => return Ok(res),
           Err(err) => {
                debug_println!("lpop {key} failed for {}",err);
                drop(conn);
                let _ = self.gain_conn();
                return Err(err);
           },
        }
    }
    pub fn rpop(&self,key: &str,count: usize)->RedisResult<Vec<String>>{
        let mut conn= self.conn.write().unwrap();
        match  conn.as_mut().unwrap().rpop::<_,Vec<String>>(key.to_string(), core::num::NonZeroUsize::new(count)){
           Ok(res) => return Ok(res),
           Err(err) => {
                debug_println!("rpop {key} failed for {}",err);
                drop(conn);
                let _ = self.gain_conn();
                return Err(err);
           },
        }
    }
}

