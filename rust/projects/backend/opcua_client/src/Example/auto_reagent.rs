use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use tokio::time::sleep;
use tokio::sync::broadcast::Receiver;
use mysql::*;
use AutoReagent::models::redis_data::RedisState as RedisData;

use crate::entity::temperature::Temperature;
use crate::debug_println;
use crate::opcua_config::data_adaptor::{collector::DataCollector,unit::DataTime::DataTime};

type MyResult<T> = Result<T,String>;

//To redis for query from front end
async fn to_redis_list(mut recv: Receiver<DataTime>,target:&'static str) -> MyResult<()>
{
    let redis_data = RedisData::new_arc();
    while let Ok(res) = recv.recv().await {
        let res = serde_json::to_string(&res).unwrap();
        redis_data.rpush_retry(target, vec![res],3).await;
    }
    Ok(())
}

async fn to_redis_str(mut recv: Receiver<DataTime>,target:&'static str)-> MyResult<()>
{
    let redis_data = RedisData::new_arc();
    while let Ok(res) = recv.recv().await {
        //only data needed
        redis_data.setex_retry(target, res.v ,9,3).await;
    }
    Ok(())
}

async fn insert_data(pool: &Pool<MySql>, data: &Vec<Temperature>, sql: &String) -> MyResult<()>
{
    let mut transaction = pool.begin().await.map_err(|e| format!("Transaction error for {e}"))?;
    for entry in data {
        sqlx::query(sql)
            .bind(entry.val)
            .bind(entry.time)
            .execute(&mut transaction)
            .await
            .map_err(|e| format!("Transaction error for {e}"))?;
    }
    transaction.commit().await.map_err(|e| format!("Transaction error for {e}"))?;
    Ok(())
}

use chrono::Datelike;
fn get_table_name_prefix() -> String {
    let now = chrono::Local::now();
    let formatted_date = now.format("%Y%m%d").to_string();
    let weekday = now.weekday().num_days_from_monday() + 1;
    let result = format!("{}_{}", formatted_date, weekday);
    debug_println!("{result}");
    result
}

//store to database
async fn flux_to_mysql(mut recv: Receiver<DataTime>,database:&'static str) -> MyResult<()>
{
    let table = get_table_name_prefix();
    sleep(std::time::Duration::from_secs(20)).await;
    let url = format!("mysql://root:121234@ayanamyrei.com:3000/{database}?ssl-mode=DISABLED");
    let creat_cmd = format!("CREATE TABLE if not exists {table}(id bigint auto_increment,val double not null,time timestamp not null,primary key(id))");
    let insert_cmd = format!("INSERT INTO {table}(val,time) VALUES (?, ?)");
    let mut records:Vec<Temperature> = vec![];
    loop {
        if let Ok(pool) = MySqlPoolOptions::new().connect(&url).await {
            if let Ok(_) = sqlx::query::<MySql>(&creat_cmd).execute(&pool).await {
                while let Ok(msg) = recv.try_recv(){ records.push(Temperature::from(msg)); }
                match insert_data(&pool, &records, &insert_cmd).await {
                    Ok(_) => {
                        debug_println!("Successfully store data to MySql");
                        records.clear(); 
                    },
                    Err(_) => debug_println!("Fail to store data to MySql"),
                }
                sleep(std::time::Duration::from_secs(300)).await;
            }else{
                debug_println!("Connect sql failed, try agian after 5s");
                sleep(std::time::Duration::from_secs(5)).await;
            }
        }else{
            debug_println!("Connect database failed, try agian after 5s");
            sleep(std::time::Duration::from_secs(5)).await;
        }
    }
}

//trim the record list to specified length with the specific time interval
async fn trim_record(max_num:i32,interval:u64,target:&'static str)-> MyResult<()>
{
    let redis_data = RedisData::new_arc();
    loop {
        match redis_data.llen(target).await {
            Ok(num) => {
                debug_println!("Redis record {target} length is {num}");
                let subtract = num - max_num;
                if subtract > 0 {
                    let _ = redis_data.lpop(target, subtract as usize).await;
                }
            },
            Err(err) => debug_println!("Trim record failed for {:?}",err),
        }
        sleep(std::time::Duration::from_secs(interval)).await;   
    }
}

async fn gain_status(collector:DataCollector<DataTime>,target:&'static str)-> MyResult<()>{
    let j1 = tokio::spawn(to_redis_str(collector.subscribe(), target));
    let j2 = tokio::spawn(DataCollector::execute_loop(collector));
    match tokio::try_join!(j1,j2) {
        Ok(_) => Ok(()),
        Err(_) => Err("Bad tokio task".to_string()),
    }
}
async fn record_to_redis_mysql(collector:DataCollector<DataTime>,target:&'static str)->MyResult<()>{
    let j1 = tokio::spawn(flux_to_mysql(collector.subscribe(),target));
    let j2 = tokio::spawn(to_redis_list(collector.subscribe(),target));
    let j3 = tokio::spawn(trim_record(3600,600,target));
    let j4 = tokio::spawn(DataCollector::execute_loop(collector));
    match tokio::try_join!(j1,j2,j3,j4) {
        Ok(_) => Ok(()),
        Err(_) => Err("Bad tokio task".to_string()),
    }
}

// business
pub async fn do_record() -> MyResult<()>{
    let sp_colletor: DataCollector<DataTime> = DataCollector::new("setpoint",3600);
    let sp_vice_colletor: DataCollector<DataTime> = DataCollector::new("setpointVice",3600);
    let switch_colletor: DataCollector<DataTime> = DataCollector::new("switch",3600);
    let switch_vice_colletor: DataCollector<DataTime> = DataCollector::new("switchVice",3600);
    let flux_colletor: DataCollector<DataTime> = DataCollector::new("flux",3600);
    let flux_vice_colletor: DataCollector<DataTime> = DataCollector::new("fluxVice",3600);
    tokio::try_join!( 
        gain_status(sp_colletor,"setpointStatus"),
        gain_status(sp_vice_colletor,"setpointViceStatus"),
        gain_status(switch_colletor,"switchStatus"),
        gain_status(switch_vice_colletor,"switchViceStatus"),
        record_to_redis_mysql(flux_colletor,"flux"),
        record_to_redis_mysql(flux_vice_colletor,"fluxVice"),
    )?;
    Ok(())
}