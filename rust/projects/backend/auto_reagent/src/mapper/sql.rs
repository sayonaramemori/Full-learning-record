use actix_web::web;
use chrono::prelude::*;
use sqlx::{pool, MySql, MySqlPool};
use crate::models::{TempRecord::TempRecord,TempRecord::DateTimeRng};
use std::collections::HashMap;
use crate::models::{redis_data::RedisState,sqlx_manager::SqlxManager};

pub async fn get_data_in_range(pool: &web::Data<SqlxManager>,time_pair:DateTimeRng,db_name:&str,table:&str) -> Vec<TempRecord<DateTime<Utc>>> {
    let pool = pool.get(db_name).unwrap();
    let query = format!("SELECT val,id,time FROM {table} WHERE time BETWEEN ? AND ?");
    let data = sqlx::query_as::<MySql, TempRecord<DateTime<Utc>>>(&query)
        .bind(time_pair.0)
        .bind(time_pair.1)
        .fetch_all(pool)
        .await;
    data.unwrap_or(vec![])
}