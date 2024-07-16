use actix_web::HttpRequest;
use std::collections::HashMap;
use actix_web::{get, web, Responder, HttpResponse};
use chrono::prelude::*;
use chrono::Duration;
use rand::Rng;
use sqlx::{pool, MySql, MySqlPool};
use crate::models::{TempRecord::TempRecord,TurbineState::TurbineState};
use super::super::AppState::RedisState;
use super::Verify::{verify,get_connection};

#[get("/findlast/{num}")]
pub async fn findlast(req: HttpRequest,num: web::Path<f64>,redis_data:web::Data<RedisState>,pool:web::Data<MySqlPool>) -> impl Responder {
    let res = verify(&req, &redis_data,&pool).await;
    if res.is_good() {
        let num :u32= num.into_inner() as u32;
        let num_str = num.to_string();
        if let Ok(mut conn) = get_connection(&redis_data).await {
            if let Ok(val) = redis::cmd("LRANGE").arg("record").arg("-".to_string() + &num_str).arg("-1").query_async::<_,Vec<String>>(&mut conn).await {
                let mut res = vec![];
                let interval = Duration::seconds(1);
                let time = Local::now().naive_local().with_nanosecond(0).unwrap();
                let time_seq = gen_time(time,interval,num);
                let val = val.into_iter().map(|v| v.parse::<f64>().unwrap() ).collect::<Vec<f64>>();
                let _ = time_seq.into_iter()
                    .rev()
                    .zip(0..num)
                    .zip(val)
                    .map(|((t,i),v)| res.push(TempRecord::new(v,i,t)))
                    .last();
                return HttpResponse::Ok().json(res);
            }else {
                println!("Failed");
            }
        }
        HttpResponse::Ok().json("Query Error")
    }else{
        HttpResponse::Unauthorized().json(res.msg())
    }
}

pub fn gen_time(start: NaiveDateTime, interval: Duration, count: u32) -> Vec<NaiveDateTime> {
    (0..count)
        .map(|i| start - interval * i as i32)
        .collect()
}

#[get("/state")]
async fn turbine_state(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<MySqlPool>) -> HttpResponse {
    let res = verify(&req, &redis_data,&pool).await;
    if res.is_good() {
        if let Ok(mut conn) = get_connection(&redis_data).await {
            let cmd = redis::cmd("HGETALL");
            let res = redis::pipe()
                .add_command(cmd.clone()).arg("turbineState:001")
                .add_command(cmd).arg("turbineState:002")
                .query_async::<_,Vec<HashMap<String,String>>>(&mut conn).await;
            if res.is_ok() {
                let res = res.unwrap();
                if res[0].is_empty()||res[1].is_empty(){ 
                    let rows = sqlx::query_as::<MySql,TurbineState>("select * from turbineState").fetch_all(pool.get_ref()).await.unwrap();
                    return HttpResponse::Ok().json(rows);
                }
                let res = res.into_iter().map(|t| TurbineState::to_turbine_state(t)).collect::<Vec<TurbineState>>();
                return HttpResponse::Ok().json(res);
            }
            else {return HttpResponse::Ok().json("Type Error"); }
        }
        HttpResponse::Ok().json("Connection to Redis failed")
    }else {
        HttpResponse::Unauthorized().json(res.msg())
    }
}