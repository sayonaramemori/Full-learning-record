use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use jwt::SignWithKey;
use sha2::Sha256;
use chrono::prelude::*;

use actix_web::{web, HttpRequest};
use crate::middleware::{redis_data::RedisState,sqlx_manager::SqlxManager};
use crate::models::entities::prelude::*;
use crate::debug_println;
use crate::models::user::exist_user;
use crate::models::token::*;

#[derive(Deserialize,Serialize,PartialEq)]
pub struct Claims{
    pub info: LoginInfo,
    pub expire_time: i64,
}
impl Claims {
    pub fn user_id_wrapper(&self) -> String {
        format!("User{}{}",self.info.username,self.expire_time.to_string())
    }
}

const SECRETKEY:&[u8] = b"sayonaramemorisdaisuki";

pub fn generate_token(info:LoginInfo,expire_time:i64) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRETKEY).unwrap();
    let body = Claims{info,expire_time};
    let claims = body.sign_with_key(&key).unwrap();
    return claims;
}

pub async fn verify_token(token:&str,redis_data: &web::Data<RedisState>,pool: &web::Data<SqlxManager>,) -> Option<Claims>{
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRETKEY).unwrap();
    match VerifyWithKey::<Claims>::verify_with_key(token, &key){
        Ok(claims) => {
            let key = claims.user_id_wrapper();
            match exist_token(&key, redis_data).await {
                Ok(token_queryed) if token_queryed == token => { 
                    debug_println!("Query in Redis"); 
                    return Some(claims);
                }
                _ => {}, 
            }
            if claims.expire_time < Utc::now().timestamp() {  return None; }
            match exist_user(&claims.info, pool).await {
                Ok(_user) => {
                    debug_println!("Store into Redis");
                    let _ = add_token(&key, token, 3600*24, redis_data).await;
                    return Some(claims);
                },
                Err(_e)=> { debug_println!("Query such user failed for {_e}",); },
            }
        },
        _ => { debug_println!("Error token"); }
    }
    None
}

pub async fn verify(req: &HttpRequest, redis_data: &web::Data<RedisState>,pool: &web::Data<SqlxManager>,) -> Option<Claims>{ 
    let token = match req.headers().get("token") {
        Some(header_value) => header_value.to_str().unwrap_or(""),
        _ => "",
    };
    if token.is_empty() {
        debug_println!("No token provided");
        return None;
    }
    verify_token(token, redis_data, pool,).await
}
