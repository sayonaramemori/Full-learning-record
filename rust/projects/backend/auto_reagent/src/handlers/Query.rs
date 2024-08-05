use std::collections::HashMap;
use actix_web::{post,get, web, Responder, HttpResponse,HttpRequest};
use std::io::Write;
use chrono::{prelude::*, Duration};
use sqlx::{pool, MySql, MySqlPool};
use crate::models::{TempRecord::TempRecord,TurbineState::TurbineState,TempRecord::{DateTimeRange,DateTimeRng,HistoryData}};
use crate::models::redis_data::RedisState;
use super::Verify::verify;
use crate::mapper::sql::get_data_in_range;

#[get("/findlastVice/{num}")]
pub async fn findlast_vice(req: HttpRequest,num: web::Path<f64>,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>) -> impl Responder {
    return findlast_record(req, num, redis_data, pool, "fluxVice").await;
}

#[get("/findlast/{num}")]
pub async fn findlast(req: HttpRequest,num: web::Path<f64>,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>) -> impl Responder {
    return findlast_record(req, num, redis_data, pool, "flux").await;
}

async fn findlast_record(req: HttpRequest,num: web::Path<f64>,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>,target:&'static str) -> impl Responder {
    let res = verify(&req, &redis_data,&pool).await;
    if res.is_good() {
        let num = num.into_inner() as i64;
        match redis_data.lrange(target, num as usize).await {
            Ok(res) => {
                let res :Vec<TempRecord<String>> = res.into_iter()
                    .zip(0..num)
                    .map(|(v,i)|{ (v,i).into() })
                    .collect();
                return HttpResponse::Ok().json(res);
            },
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string()+" leading operation fail"),
        };
    }else{
        HttpResponse::Unauthorized().json(res.msg())
    }
}

#[get("/state")]
async fn turbine_state(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<MySqlPool>) -> HttpResponse {
    let res = verify(&req, &redis_data,&pool).await;
    if res.is_good() {
        match redis_data.hgetall::<HashMap<String,String>>(vec!["turbineState:001","turbineState:002"]).await {
            Ok(res) => {
                let mut response:Vec<TurbineState> = vec![];
                for i in res {
                    if !i.is_empty() {
                        response.push(TurbineState::to_turbine_state(i));
                    }
                }
                return HttpResponse::Ok().json(response);
            },
            Err(e) => return HttpResponse::InternalServerError().json(e.to_string()+" leading operation fail"),
        }
    }
    HttpResponse::Unauthorized().json(res.msg())
}

#[post("/history")]
async fn history(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<MySqlPool>,data:web::Json<DateTimeRange>) -> HttpResponse {
    let res = verify(&req, &redis_data,&pool).await;
    if res.is_good() {
        if let (Ok(start),Ok(end)) = (data.start.parse::<DateTime<Utc>>(),data.end.parse::<DateTime<Utc>>()) {
            let offset = FixedOffset::east_opt(8*3600).unwrap();
            let start = start.with_timezone(&offset).naive_local();
            let end = end.with_timezone(&offset).naive_local();
            let time_pair = DateTimeRng(start,end);
            let res = get_data_in_range(&pool, time_pair).await;
            let res: Vec<TempRecord<NaiveDateTime>>= res.into_iter().map(|v|{TempRecord{time:v.time.naive_local(),val:v.val,id:v.id}}).collect();
            let mut temp = 0.0;
            res.iter().map(|i| {temp = temp + i.val;}).last();
            let average = temp/(res.len() as f64);
            let total_time = (res.len() as f64)/3600.0;
            let res = HistoryData {average,total_time,records:res};
            HttpResponse::Ok().json(res)
        }else{
            HttpResponse::InternalServerError().json("Connection to Redis failed")
        }
    }else {
        HttpResponse::Unauthorized().json(res.msg())
    }
}
