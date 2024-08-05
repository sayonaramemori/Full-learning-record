use super::Verify::verify;
use actix_web::{post,get, web, Responder, HttpResponse,HttpRequest};
use sqlx::MySqlPool;
use crate::debug_println;
use crate::models::redis_data::{self, RedisState};
use crate::models::Operation;


pub async fn start_or_stop(num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>,op:Operation) -> HttpResponse {
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_good() {
        let number = num.into_inner();
        let obj = match number {
            0 => "switch",
            1 => "switchVice",
            _ => return HttpResponse::BadRequest().json("Bad params"),
        };
        let op = op.to_bool().to_string();
        match redis_data.lpush_ex(obj,vec![op],10).await {
           Ok(_) => return HttpResponse::Ok().json("Operation success"),
           Err(e) => {
                debug_println!("Connect redis failed for {e}");
                return HttpResponse::InternalServerError().json(e.to_string() + " leading Operation failed");
            },
        }
    }
    HttpResponse::Unauthorized().json(res.msg())
}

#[get("/start/{num}")]
pub async fn start(num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>) -> HttpResponse {
    return start_or_stop(num, req, redis_data, pool, Operation::Start).await;
}

#[get("/stop/{num}")]
pub async fn stop(num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>) -> HttpResponse {
    return start_or_stop(num, req, redis_data, pool, Operation::Stop).await;
}

#[get("/pumpStatus/{num}")]
pub async fn pump_status(num:web::Path<u32>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>) -> HttpResponse {
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_good() {
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
    HttpResponse::Unauthorized().json(res.msg())
}

#[get("/setpoint/{num}/{sp}")]
pub async fn set_point(nums:web::Path<(u32,f64)>,req:HttpRequest,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>) -> HttpResponse{
    let res = verify(&req, &redis_data, &pool).await;
    if res.is_good() {
        let (num,sp) = nums.into_inner();
        let obj = match num {
            0 => "setpoint",
            1 => "setpointVice",
            _ => return HttpResponse::BadRequest().json("Bad params"),
        };
        if sp<0.0 || sp>200.0 {return HttpResponse::BadRequest().json("Bad params");}
        match redis_data.lpush_ex(obj, vec![sp.to_string()], 10).await {
            Ok(_) => return HttpResponse::Ok().json("Operation success"),
            Err(e) => {
                debug_println!("Connect redis failed for {e}");
                return HttpResponse::InternalServerError().json(e.to_string() + " leading Operation failed")
            },
        }
    }
    HttpResponse::Unauthorized().json(res.msg())
}
