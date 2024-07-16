use super::Verify::{get_connection,verify};
use actix_web::{get, web, Responder, HttpResponse,HttpRequest};
use sqlx::MySqlPool;
use super::super::AppState::RedisState;

#[get("/startMain")]
pub async fn startMain(req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>) -> HttpResponse {
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_good() {
        
        HttpResponse::Unauthorized().json(res.msg())
    }else{
        HttpResponse::Unauthorized().json(res.msg())
    }
}
