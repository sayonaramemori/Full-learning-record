use actix_web::{http::header,get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};
use mysql::binlog::{row, value};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use sqlx::{MySql, MySqlPool};
use crate::models::{LoginInfo::LoginInfo,TurbineState::TurbineState};
use cookie::Cookie;
use std::sync::Arc;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use sha2::Sha256;
use super::super::AppState::RedisState;

struct StatusMsg(bool,String);
const SECRETKEY :&[u8;8]= b"sayonara";

//return (bool,msg) msg is error msg when bool is false and msg is username when bool is true
async fn verify(req: &HttpRequest, redis_data: &web::Data<RedisState>) -> StatusMsg { 
    let token = match req.headers().get("Authorization") {
        Some(header_value) => header_value.to_str().unwrap_or(""),
        None => "",
    };
    if token.is_empty() {
        return StatusMsg(false,"No token provided".to_string());
    }
    let key: Hmac<Sha256> = Hmac::new_from_slice(SECRETKEY).unwrap();
    match VerifyWithKey::<(LoginInfo,NaiveDateTime)>::verify_with_key(token, &key){
      Ok((LoginInfo {username,..},_)) => {
        let mut conn = get_connection(&redis_data).await.unwrap();
        // let token_stored: String = 
        match conn.get::<_,String>(&username).await {
            Ok(token_stored) => {
                if token_stored != token {
                    return StatusMsg(false,"Expired token".to_string());
                }else {
                    return StatusMsg(true,username);
                }
            },
            Err(err) => {
                return StatusMsg(false,"Error or Expired token".to_string());
            },
        }
      },
      _ => StatusMsg(false,"Bad token".to_string()),
    }
}

async fn get_connection(client: &RedisState) ->Result<MultiplexedConnection, redis::RedisError> {
    let mut conn = client.redis_client.get_multiplexed_async_connection().await?;
    redis::cmd("AUTH")
        .arg(&client.redis_passwd)
        .query_async(&mut conn)
        .await?;
    Ok(conn)
}

#[post("/login")]
async fn login(info: web::Json<LoginInfo>, redis_data: web::Data<RedisState>, pool:web::Data<MySqlPool>) -> HttpResponse {
    let user_info = sqlx::query_as::<MySql,LoginInfo>("select username, password from admin where username=? and password=?")
        .bind(&info.username)
        .bind(&info.password)
        .fetch_one(pool.get_ref())
        .await;
    match user_info {
        Ok(user_info) => match get_connection(&redis_data).await{
            Ok(mut connection) => {
                let key: Hmac<Sha256> = Hmac::new_from_slice(SECRETKEY).unwrap();
                let claims = (user_info,Utc::now().naive_local());
                let token = claims.sign_with_key(&key).unwrap();
                match redis::cmd("SETEX").arg(&info.username).arg(3600).arg(&token).query_async::<_, ()>(&mut connection).await {
                    Ok(_) => {
                        let cookie = Cookie::build(("token", token))
                            .path("/")
                            .http_only(true)
                            .build();
                        HttpResponse::Ok()
                            .append_header((header::SET_COOKIE, cookie.to_string()))
                            .body("Login successful")
                    },
                    Err(err) => {
                        return HttpResponse::InternalServerError().body("Internal Server Error");
                    },
                }
            },
            Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
        },
        _ => HttpResponse::Unauthorized().json("Invalid username or password"),
    }
}

#[get("/logout")]
async fn logout(req: HttpRequest, redis_data: web::Data<RedisState>) -> HttpResponse {
    let res = verify(&req, &redis_data).await;
    if res.0 {
        if let Ok(mut conn) = get_connection(&redis_data).await {
            redis::cmd("DEL").arg(res.1).query_async::<_,()>(&mut conn).await.unwrap();
            return HttpResponse::Ok().json("Logged out");
        }
    }
    HttpResponse::Unauthorized().json(res.1)
}

#[get("/verify")]
async fn check_privilege(req: HttpRequest, redis_data: web::Data<RedisState>) -> HttpResponse {
    let res = verify(&req, &redis_data).await;
    if res.0 {
        HttpResponse::Ok().json(res.1)
    }else {
        HttpResponse::Unauthorized().json(res.1)
    }
}

#[get("/state")]
async fn state(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<MySqlPool>) -> HttpResponse {
    let res = verify(&req, &redis_data).await;
    if res.0 {
        let rows = sqlx::query_as::<MySql,TurbineState>("select * from turbineState").fetch_all(pool.get_ref()).await.unwrap();
        HttpResponse::Ok().json(rows)
    }else {
        HttpResponse::Unauthorized().json(res.1)
    }
}





