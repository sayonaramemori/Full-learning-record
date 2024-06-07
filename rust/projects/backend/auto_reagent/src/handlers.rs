use actix_web::{get, web, Responder, HttpResponse};
use crate::models::*;
use chrono::prelude::*;
use chrono::Duration;
use rand::Rng;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/findlast/{num}")]
pub async fn findlast(num: web::Path<u32>) -> impl Responder {
    let mut res = vec![];
    let num :u32= num.into_inner();
    let mut rng = rand::thread_rng();
    let time = Local::now().naive_local();
    let time :NaiveDateTime = time.with_nanosecond(0).unwrap();
    let interval = Duration::seconds(1);
    let time_seq = gen_time(time,interval,num);
    let _ = time_seq.into_iter()
        .rev()
        .zip(0..num)
        .map(|(val,i)| res.push(TempRecord::new(rng.gen_range(0.0..100.0),i,val)))
        .last();
    HttpResponse::Ok().json(res)
}


