use serde::{Deserialize, Serialize};
use chrono::prelude::*;

use actix_web::{web, HttpRequest};
use crate::middleware::{redis_data::RedisState,sqlx_manager::SqlxManager};
use crate::models::entities::prelude::*;
use crate::debug_println;
use crate::models::token::*;
use crate::middleware::token_man::TokenManager;


pub fn generate_token(info: &LoginInfo,expire_time: chrono::TimeDelta) -> String {
    TokenManager::new().generate_token_with_time(info,expire_time)
}

pub async fn verify_token(token: &str,redis_data: &web::Data<RedisState>,) -> Option<LoginInfo>{
    let token_man = TokenManager::new();
    if let Some(info) = token_man.unravel_with_time_check::<LoginInfo>(token){
        match exist_token(&info.username, redis_data).await {
            Ok(token_queryed) if token_queryed == token => { 
                debug_println!("Query in Redis"); 
                return Some(info);
            }
            _ => {}, 
        }
    }
    None
}

pub async fn verify(req: &HttpRequest, redis_data: &web::Data<RedisState>) -> Option<LoginInfo>{ 
    let token = match req.headers().get("token") {
        Some(header_value) => header_value.to_str().unwrap_or(""),
        _ => "",
    };
    if token.is_empty() {
        debug_println!("No token provided");
        return None;
    }
    verify_token(token, redis_data,).await
}
