use actix_web::{post,get, web, Responder, HttpResponse,HttpRequest};
use chrono::prelude::*;
use crate::models::entities::prelude::*;
use crate::handlers::entities::date_time_range::*;
use crate::middleware::{redis_data::RedisState,sqlx_manager::SqlxManager};
use super::verify_token::verify;
use crate::models::query_data::get_data_in_range;
use super::entities::history_data::HistoryData;
use chrono::Datelike;

#[post("/historyMain")]
async fn main_history(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<SqlxManager>,data:web::Json<StringDateTimeRng>) -> HttpResponse {
    return history(req,redis_data,pool,data,"flux").await;
}

#[post("/historyVice")]
async fn vice_history(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<SqlxManager>,data:web::Json<StringDateTimeRng>) -> HttpResponse {
    return history(req,redis_data,pool,data,"fluxVice").await;
}

fn get_table_name_prefix() -> String {
    let now = chrono::Local::now();
    let formatted_date = now.format("%Y%m%d").to_string();
    let weekday = now.weekday().num_days_from_monday() + 1;
    format!("{}_{}", formatted_date, weekday)
}

async fn history(req: HttpRequest, redis_data: web::Data<RedisState>,pool: web::Data<SqlxManager>,data:web::Json<StringDateTimeRng>,table:&'static str) -> HttpResponse {
    let res = verify(&req, &redis_data,&pool).await;
    if res.is_some() {
        if let (Ok(start),Ok(end)) = (data.start.parse::<DateTime<Utc>>(),data.end.parse::<DateTime<Utc>>()) {
            let offset = FixedOffset::east_opt(8*3600).unwrap();
            let start = start.with_timezone(&offset).naive_local();
            let end = end.with_timezone(&offset).naive_local();
            let time_pair = NaiveDateTimeRng(start,end);
            let res = get_data_in_range(&pool, time_pair,table,&get_table_name_prefix()).await;
            if res.is_err() {return HttpResponse::InternalServerError().json("Error in sql") }
            let res = res.unwrap();
            println!("Len is {}",res.len());

            let step = res.len()/250;
            let mut res_kept_iter = res.into_iter();
            let mut res:Vec<TempRecord<NaiveDateTime>> = vec![];
            while let Some(item) = res_kept_iter.nth(step) { res.push(item.into()); }

            let mut temp = 0.0;
            res.iter().map(|i| {temp = temp + i.val;}).last();
            let average = temp/(res.len() as f64);
            let total_time = (res.len() as f64)/3600.0;

            let res = HistoryData {average,total_time,records:res};
            HttpResponse::Ok().json(res)
        }else{
            HttpResponse::InternalServerError().json("Bad Time Range")
        }
    }else {
        HttpResponse::Unauthorized().json("Bad token")
    }
}


