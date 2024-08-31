use actix_web::{web, HttpRequest};
use jwt::token;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, MySqlPool};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use jwt::SignWithKey;
use sha2::Sha256;
use chrono::prelude::*;

use crate::models::redis_data::RedisState;
use crate::models::LoginInfo::LoginInfo;
use crate::debug_println;

#[derive(Deserialize,Serialize,PartialEq)]
pub struct Claims{
    pub info: LoginInfo,
    pub expire_time: i64,
}

pub const SECRETKEY :&[u8;8]= b"sayonara";


pub struct StatusMsg(bool,String);
impl StatusMsg {
    pub fn is_good(&self) -> bool{ return self.0; }
    pub fn msg(&self) -> String { return self.1.clone(); }
    pub fn parsed_name(&self) -> String {return self.1.clone();}
}

pub fn generate_token(info:LoginInfo,expire_time:i64) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRETKEY).unwrap();
    let body = Claims{info,expire_time};
    let claims = body.sign_with_key(&key).unwrap();
    return claims;
}

//Query DB first then cache the token to Redis
pub async fn exist_user(info: &LoginInfo,redis_data: &web::Data<RedisState>,pool: &web::Data<MySqlPool>,token:Option<&str>) -> StatusMsg{
    let res = sqlx::query_as::<MySql,LoginInfo>("select username, password from admin where username=? and password=?")
        .bind(&info.username)
        .bind(&info.password)
        .fetch_one(pool.get_ref())
        .await;
    if let Ok(_user_info) = res {
        debug_println!("Query user in DB");
        if let Some(token)= token {
            let _ = redis_data.setex(token, 1, 3600).await;
        }
        return StatusMsg(true,String::from(""));
    }else{
        return StatusMsg(false,String::from("Error User Info"))
    }
}

pub async fn verify_token(token:&str,redis_data: &web::Data<RedisState>,pool: &web::Data<MySqlPool>) -> StatusMsg{
    //Query Redis first
    if let Ok(res) = redis_data.get(token).await {
        debug_println!("Query user in Redis");
        if !res.is_empty(){ return StatusMsg(true,String::from("Ok")); }
    }
    //Inspect the token
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRETKEY).unwrap();
    match VerifyWithKey::<Claims>::verify_with_key(token, &key){
        Ok(claims) => {
            let res = exist_user(&claims.info, redis_data, pool,Some(token)).await;
            if res.is_good(){
                let now = Utc::now().timestamp();
                if claims.expire_time > now { return StatusMsg(true,claims.info.username); }
                else { return StatusMsg(false,String::from("Expired token")); }
            }else{
                return res;
            }
        },
        _ => return StatusMsg(false,String::from("Bad token")),
    }
}

// return (bool,msg) in which msg is error msg 
// when bool is false and msg is username when bool is true for the first time to query
pub async fn verify(req: &HttpRequest, redis_data: &web::Data<RedisState>,pool: &web::Data<MySqlPool>) -> StatusMsg { 
    let token = match req.headers().get("token") {
        Some(header_value) => header_value.to_str().unwrap_or(""),
        None => "",
    };
    if token.is_empty() { return StatusMsg(false,"No token provided".to_string()); }
    let res = verify_token(token, redis_data, pool).await;
    return res;
}

pub async fn get_connection(client: &RedisState) ->Result<MultiplexedConnection, redis::RedisError> {
    let mut conn = client.redis_client.get_multiplexed_async_connection().await?;
    redis::cmd("AUTH").arg(&client.redis_passwd).query_async(&mut conn).await?;
    Ok(conn)
}