use actix_web::{get, web, Responder, HttpResponse};
use actix_session::{Session};
use chrono::prelude::*;
use chrono::Duration;
use rand::Rng;
use crate::models::TempRecord::TempRecord;

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/findlast/{num}")]
pub async fn findlast(num: web::Path<f64>,session:Session) -> impl Responder {
    println!("{:?}",session.get::<String>("privilege_string").unwrap());
    let mut res = vec![];
    let num :u32= num.into_inner() as u32;
    let mut rng = rand::thread_rng();
    let time = Local::now().naive_local().with_nanosecond(0).unwrap();
    let interval = Duration::seconds(1);
    let time_seq = gen_time(time,interval,num);
    let _ = time_seq.into_iter()
        .rev()
        .zip(0..num)
        .map(|(val,i)| res.push(TempRecord::new(rng.gen_range(0.0..100.0),i,val)))
        .last();
    HttpResponse::Ok().json(res)
}

pub fn gen_time(start: NaiveDateTime, interval: Duration, count: u32) -> Vec<NaiveDateTime> {
    (0..count)
        .map(|i| start - interval * i as i32)
        .collect()
}
