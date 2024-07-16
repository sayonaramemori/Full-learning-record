use actix_web::{get, post, web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::{MySql, MySqlPool};
use crate::models::LoginInfo::LoginInfo;
use super::super::AppState::RedisState;
use super::Verify::{get_connection,verify,exist_user,generate_token};


#[post("/login")]
async fn login(info: web::Json<LoginInfo>, redis_data: web::Data<RedisState>, pool:web::Data<MySqlPool>) -> HttpResponse {
    let res = exist_user(&info, &redis_data, &pool).await;
    let user_info = LoginInfo{username:info.username.clone(),password:info.password.clone()};
    if res.is_good() {
        let VeryfyInterval = Duration::hours(24);
        return HttpResponse::Ok().json(generate_token(user_info,Utc::now().checked_add_signed(VeryfyInterval).unwrap().timestamp()));
    }else{
        return HttpResponse::Unauthorized().json(res.msg());
    }
    
}

#[get("/logout")]
async fn logout(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<MySqlPool>) -> HttpResponse {
    let res = verify(&req, &redis_data,&pool).await;
    if res.is_good() {
        if let Ok(mut conn) = get_connection(&redis_data).await {
            redis::cmd("DEL").arg(res.parsed_name()).query_async::<_,()>(&mut conn).await.unwrap();
            return HttpResponse::Ok().json("Logged out");
        }
    }
    HttpResponse::Unauthorized().json(res.msg())
}

#[get("/verify")]
async fn check_privilege(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<MySqlPool>) -> HttpResponse {
    let res = verify(&req, &redis_data,&pool).await;
    if res.is_good() {
        HttpResponse::Ok().json(res.parsed_name())
    }else {
        HttpResponse::Unauthorized().json(res.msg())
    }
}


