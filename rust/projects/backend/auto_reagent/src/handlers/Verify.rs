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
impl Claims {
    pub fn user_id_wrapper(&self) -> String {
        //device imformation should be added
        format!("User{}{}",self.info.username,self.expire_time.to_string())
    }
}

pub const SECRETKEY :&[u8;8]= b"sayonara";

pub fn user_id_wrapper(username:&str,timestamp:i64) -> String {
    //device imformation should be added
    format!("User{username}{}",timestamp.to_string())
}

pub fn generate_token(info:LoginInfo,expire_time:i64) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRETKEY).unwrap();
    let body = Claims{info,expire_time};
    let claims = body.sign_with_key(&key).unwrap();
    return claims;
}

//Query DB first then cache the token to Redis,return user id
pub async fn exist_user(info: &LoginInfo,pool: &web::Data<MySqlPool>) -> Option<LoginInfo>{
    let res = sqlx::query_as::<MySql,LoginInfo>("select id,username, password from admin where username=? and password=?")
        .bind(&info.username)
        .bind(&info.password)
        .fetch_one(pool.get_ref())
        .await;
    if let Ok(info) = res {
        debug_println!("Query user in DB Success");
        Some(info)
    }else{
        debug_println!("Query user in DB Failed");
        None
    }
}

pub async fn verify_token(token:&str,redis_data: &web::Data<RedisState>,pool: &web::Data<MySqlPool>) -> Option<Claims>{
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRETKEY).unwrap();
    match VerifyWithKey::<Claims>::verify_with_key(token, &key){
        Ok(claims) => {
            let key = user_id_wrapper(&claims.info.username, claims.expire_time);
            //check in redis first
            match redis_data.get(&key).await {
                Ok(res) => { if res == token {debug_println!("Query in Redis"); return Some(claims);}else{debug_println!("Not equal")}}
                Err(_e) => {}, 
            }
            let now = Utc::now().timestamp();
            //Well Decoded token, then check the information
            if claims.expire_time < now {  
                debug_println!("Expired Token");
                return None;
            }
            match exist_user(&claims.info, pool).await {
                Some(user) => {
                    //Trust this token and store it in Redis for quick query
                    if user == claims.info {
                        debug_println!("Store into Redis");
                        let _ = redis_data.setex(&key, token, 3600*24).await;
                        return Some(claims);
                    }else{ debug_println!("Cheating Token"); }
                },
                None => { debug_println!("No such user"); },
            }
        },
        _ => { debug_println!("Error token"); }
    }
    None
}

pub async fn verify(req: &HttpRequest, redis_data: &web::Data<RedisState>,pool: &web::Data<MySqlPool>) -> Option<Claims>{ 
    let token = match req.headers().get("token") {
        Some(header_value) => header_value.to_str().unwrap_or(""),
        _ => "",
    };
    if token.is_empty() {
        debug_println!("No token provided");
        return None;
    }
    verify_token(token, redis_data, pool).await
}

pub async fn get_connection(client: &RedisState) ->Result<MultiplexedConnection, redis::RedisError> {
    let mut conn = client.redis_client.get_multiplexed_async_connection().await?;
    redis::cmd("AUTH").arg(&client.redis_passwd).query_async(&mut conn).await?;
    Ok(conn)
}