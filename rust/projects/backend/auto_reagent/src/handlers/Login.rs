use actix_web::{get, post, web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use crate::models::LoginInfo::LoginInfo;
use super::Verify::{get_connection,verify,exist_user,generate_token};
use crate::models::{redis_data::RedisState,sqlx_manager::SqlxManager};


#[post("/login")]
async fn login(info: web::Json<LoginInfo>, _redis_data: web::Data<RedisState>, pool:web::Data<SqlxManager>) -> HttpResponse {
    let res = exist_user(&info, &pool).await;
    if res.is_some() {
        let verify_interval = Duration::hours(24);
        let user_info = info.into_inner();
        HttpResponse::Ok()
        .insert_header(("token",generate_token(user_info,Utc::now().checked_add_signed(verify_interval).unwrap().timestamp())))
        .body("Ok")
    }else{
        HttpResponse::Unauthorized().body("Bad User Info")
    }
}

#[get("/logout")]
async fn logout(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<SqlxManager>) -> HttpResponse {
    let res = verify(&req, &redis_data,&pool).await;
    if let Some(res) = res{
        if let Ok(mut conn) = get_connection(&redis_data).await {
            redis::cmd("DEL").arg(res.user_id_wrapper()).query_async::<_,()>(&mut conn).await.unwrap();
            return HttpResponse::Ok().json("Logged out");
        }
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


