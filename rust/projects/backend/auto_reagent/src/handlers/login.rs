use actix_web::http::header;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use crate::models::entities::prelude::LoginInfo;
use super::verify_token::{verify,generate_token};
use crate::middleware::{redis_data::RedisState,sqlx_manager::SqlxManager};
use crate::models::user::exist_user;
use crate::models::token::del_token;

#[post("/login")]
async fn login(info: web::Json<LoginInfo>, _redis_data: web::Data<RedisState>, pool:web::Data<SqlxManager>) -> HttpResponse {
    let res = exist_user(&info, &pool).await;
    if res.is_ok() {
        let verify_interval = Duration::hours(24);
        let user_info = info.into_inner();
        let header = ("token",generate_token(user_info,Utc::now().checked_add_signed(verify_interval).unwrap().timestamp()));
        HttpResponse::Ok()
            .insert_header(header)
            .body("Ok")
    }else{
        HttpResponse::Unauthorized().body("Bad User Info")
    }
}

#[get("/logout")]
async fn logout(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<SqlxManager>) -> HttpResponse {
    let res = verify(&req, &redis_data,&pool).await;
    if let Some(res) = res{
        let _ = del_token(&res.user_id_wrapper(), &redis_data);
        return HttpResponse::Ok().json("Logged out");
    }
    HttpResponse::Unauthorized().body("Bad token")
}

#[get("/verify")]
async fn check_privilege(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<SqlxManager>) -> HttpResponse {
    let res = verify(&req, &redis_data,&pool).await;
    if let Some(res) = res {
        HttpResponse::Ok().json(res.info.username)
    }else {
        HttpResponse::Unauthorized().body("Bad token")
    }
}


