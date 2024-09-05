use super::Verify::verify;
use actix_web::{post,get, web, Responder, HttpResponse,HttpRequest};
use sqlx::MySqlPool;
use crate::debug_println;
use crate::models::redis_data::{self, RedisState};
use crate::models::Operation;
use crate::websocket::myws::*;

use actix::prelude::*;
use std::collections::HashMap;
use std::sync::{RwLock,Arc};

pub async fn start_or_stop(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<HashMap<String,MySqlPool>>,op:Operation) -> HttpResponse {
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_some() {
        let number = num.into_inner();
        let target = match number {
            0 => "switch",
            1 => "switchVice",
            _ => return HttpResponse::BadRequest().json("Bad params"),
        };
        match op{
            Operation::Start => return start(target.to_string(),data).await,
            Operation::Stop => return stop(target.to_string(),data).await,
        }
    }
    HttpResponse::Unauthorized().json("Bad Token")
}

#[get("/start/{num}")]
pub async fn start_operation(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<HashMap<String,MySqlPool>>,) -> HttpResponse {
    start_or_stop(data, num, req, redis_data, pool, Operation::Start).await
}

#[get("/stop/{num}")]
pub async fn stop_operation(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<HashMap<String,MySqlPool>>) -> HttpResponse {
    start_or_stop(data, num, req, redis_data, pool, Operation::Stop).await
}

#[get("/pumpStatus/{num}")]
pub async fn pump_status(num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<HashMap<String,MySqlPool>>) -> HttpResponse {
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
pub async fn set_point(data: web::Data<Arc<RwLock<Vec<Addr<MyWs>>>>>,nums:web::Path<(u32,f64)>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<HashMap<String,MySqlPool>>) -> HttpResponse{
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
