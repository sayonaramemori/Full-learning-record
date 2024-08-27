use actix_web::web;
use chrono::prelude::*;
use sqlx::{pool, MySql, MySqlPool};
use crate::models::{TempRecord::TempRecord,TempRecord::DateTimeRng};

pub async fn get_data_in_range(pool: &web::Data<MySqlPool>,time_pair:DateTimeRng,table:&str) -> Vec<TempRecord<DateTime<Utc>>> {
    let query = format!("SELECT val,id,time FROM {table} WHERE time BETWEEN ? AND ?");
    let data = sqlx::query_as::<MySql, TempRecord<DateTime<Utc>>>(&query)
        .bind(time_pair.0)
        .bind(time_pair.1)
        .fetch_all(pool.as_ref())
        .await;
        data.unwrap()
}