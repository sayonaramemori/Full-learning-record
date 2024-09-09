use super::Verify::verify;
use actix_web::{post,get, web, Responder, HttpResponse,HttpRequest};
use crate::models::{redis_data::RedisState,sqlx_manager::SqlxManager};
use crate::models::Operation;
use crate::websocket::myws::*;

use actix::prelude::*;
use std::sync::{RwLock,Arc};

#[get("/startMain")]
pub async fn start_main(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<SqlxManager>,) -> HttpResponse {
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_some() {
        let target = "switch";
        return start(target.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

#[get("/startVice")]
pub async fn start_vice(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<SqlxManager>,) -> HttpResponse {
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_some() {
        let target = "switchVice";
        return start(target.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

#[get("/stopMain")]
pub async fn stop_main(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<SqlxManager>,) -> HttpResponse {
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_some() {
        let target = "switch";
        return stop(target.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

#[get("/stopVice")]
pub async fn stop_vice(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<SqlxManager>,) -> HttpResponse {
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_some() {
        let target = "switchVice";
        return stop(target.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

#[get("/pumpStatus/{num}")]
pub async fn pump_status(num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<SqlxManager>) -> HttpResponse {
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_some() {
        let number = num.into_inner();
        let obj = match number {
            0 => "switchStatus",
            1 => "switchViceStatus",
            2 => "setpointStatus",
            3 => "setpointViceStatus",
            _ => return HttpResponse::BadRequest().json("Bad params"),
        };
        match redis_data.get(obj).await {
            Ok(res) => return HttpResponse::Ok().json(res),
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string() + " leading Operation failed"),
        }
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

#[get("/setpoint/{num}/{sp}")]
pub async fn set_point(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,nums:web::Path<(u32,f64)>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<SqlxManager>) -> HttpResponse{
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_some() {
        let (num,sp) = nums.into_inner();
        let obj = match num {
            0 => "setpoint",
            1 => "setpointVice",
            _ => return HttpResponse::BadRequest().json("Bad params"),
        };
        if sp<0.0 || sp>200.0 {return HttpResponse::BadRequest().json("Bad params");}
        return setpoint(obj.to_string(),sp.to_string(),data).await;
    }
    HttpResponse::Unauthorized().json("Bad Token")
}
