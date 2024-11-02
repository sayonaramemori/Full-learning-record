use actix_web::{get, post, web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use crate::models::entities::prelude::LoginInfo;
use super::verify_token::{verify,generate_token};
use crate::middleware::{redis_data::RedisState,sqlx_manager::SqlxManager};
use crate::models::user::exist_user;
use crate::models::token::{del_token,add_token};

#[post("/login")]
async fn login(info: web::Json<LoginInfo>, redis_data: web::Data<RedisState>, pool:web::Data<SqlxManager>) -> HttpResponse {
    let res = exist_user(&info, &pool).await;
    if res.is_ok() {
        let verify_interval = Duration::hours(24);
        let user_info = info.into_inner();
        let token = generate_token(&user_info,verify_interval);
        let _ = add_token(&user_info.username, &token, 3600*24, &redis_data).await;
        let header = ("token",token);
        HttpResponse::Ok()
            .insert_header(header)
            .body("Ok")
    }else{
        HttpResponse::Unauthorized().body("Bad User Info")
    }
}

#[get("/logout")]
async fn logout(req: HttpRequest, redis_data: web::Data<RedisState>,) -> HttpResponse {
    let res = verify(&req, &redis_data).await;
    if let Some(res) = res{
        let _ = del_token(&res.username, &redis_data).await;
        return HttpResponse::Ok().json("Logged out");
    }
    HttpResponse::Unauthorized().body("Bad token")
}

#[get("/verify")]
async fn check_privilege(req: HttpRequest, redis_data: web::Data<RedisState>) -> HttpResponse {
    let res = verify(&req, &redis_data).await;
    if let Some(res) = res {
        HttpResponse::Ok().json(res.username)
    }else {
        HttpResponse::Unauthorized().body("Bad token")
    }
}


