use crate::store::middleware::DataStore;
use lazy_static::lazy_static;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, MySqlPool,Pool};
use tokio::time::sleep;
use tokio::signal;
use tokio::task::{self,spawn_blocking};
use tokio::sync::broadcast::{self,Sender,Receiver};

use std::sync::Arc;
use mysql::prelude::Queryable;
use mysql::*;
use crate::store::opcuaSession::*;
use AutoReagent::models::redis_data::RedisState as RedisData;
use crate::Models::Temperature::Temperature;
use crate::opcua_config::node_config::NodeConfig;
use crate::debug_println;

//Expose to be called by websocket client
pub async fn transfer_data_to_plc(ds:Arc<DataStore>,target:String,val:String) {
    let config = ds.get::<NodeConfig>().unwrap();
    let redis_data = ds.get::<RedisData>().unwrap();
    let status_key = target.to_string() + "Status";
    let session_better = OpcuaSession::new_arc().await;
    let id= config.node(&target);
    if let Some(id) = id {
        if OpcuaSession::async_write_single_retry(session_better, id, config.get_variant(&target,val.clone()).unwrap(), 3).await
        {
            redis_data.setex_retry(&status_key, val,10,5).await;
        }
    }
}

//collect datas from a node with 1 sec time interval
async fn collect_data(target:&'static str,sender: Sender<DataTime>){
    let ds = create_data_store(false,false,true,true).await;
    let config = ds.get::<NodeConfig>().unwrap();
    loop {
        let session_better = ds.get::<OpcuaSession>().unwrap();
        let flux = config.node(target).unwrap();
        let res = OpcuaSession::async_read_single(session_better, flux).await;
        match res{
            Ok(res) => {
                if let Some(res) = res  {
                    let _ = sender.send(res);
                }
            },
            _ => debug_println!("Reading node failed"),
        };
        sleep(std::time::Duration::from_secs(1)).await;
    }
}

// 1. Unavailable Mysql raise error 
// 2. Config file not exists leading panic
// 3. Unreachable opcua server raises error
pub async fn create_data_store(redis:bool,mysql:bool,opcua_config:bool,opcua_session:bool,)->Arc<DataStore> {
    let ds = Arc::new(DataStore::new());
    if redis {
        let redis_data = RedisData::new("Iloveyouxuwu121234","redis://:Iloveyouxuwu121234@kazusa.vip", );
        ds.insert(redis_data);
    }
    if mysql {
        let url = "mysql://root:121234@kazusa.vip:3000/plc?ssl-mode=DISABLED";
        let pool = MySqlPoolOptions::new().connect(url).await.unwrap();
        ds.insert(pool);
    }
    if opcua_config {
        let config = NodeConfig::new().await;
        ds.insert(config);
    }
    if opcua_session {
        let session = OpcuaSession::new("opc.tcp://127.0.0.1:49320").await;
        ds.insert(session);
    }
    ds
}

//To redis for query from front end
async fn to_redis_list(mut recv: Receiver<DataTime>,target:&'static str) {
    let ds = create_data_store(true, false, false, false).await;
    let redis_data = ds.get::<RedisData>().unwrap();
    while let Ok(res) = recv.recv().await {
        redis_data.rpush_retry(target, vec![res.to_string()],3).await;
    }
}

async fn to_redis_str(mut recv: Receiver<DataTime>,target:&'static str) {
    let ds = create_data_store(true, false, false, false).await;
    let redis_data = ds.get::<RedisData>().unwrap();
    while let Ok(res) = recv.recv().await {
        redis_data.setex_retry(target, res.to_string(),9,3).await;
    }
}

async fn insert_data(pool: &Pool<MySql>, data: &Vec<Temperature>, sql: &String) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;
    for entry in data {
        sqlx::query(sql)
            .bind(entry.val)
            .bind(entry.time)
            .execute(&mut transaction)
            .await?;
    }
    transaction.commit().await?;
    Ok(())
}

//store to database
async fn flux_to_mysql(mut recv: Receiver<DataTime>,table:&'static str) {
    sleep(std::time::Duration::from_secs(20)).await;
    let ds = create_data_store(false,true,false,false).await;
    let creat_cmd = format!("CREATE TABLE if not exists {table}(id bigint auto_increment,val double not null,time timestamp not null,primary key(id))");
    let insert_cmd = format!("INSERT INTO {table}(val,time) VALUES (?, ?)");
    let mut records:Vec<Temperature> = vec![];
    loop {
        let pool = ds.get::<Pool<MySql>>().unwrap();
        if let Ok(_) = sqlx::query::<MySql>(&creat_cmd).execute(pool.as_ref()).await {
            while let Ok(msg) = recv.try_recv(){ records.push(Temperature::from(msg)); }
            match insert_data(&pool, &records, &insert_cmd).await {
                Ok(_) => {
                    records.clear();
                    debug_println!("Successfully store data to MySql")
                },
                Err(_) => debug_println!("Fail to store data to MySql"),
            }
            sleep(std::time::Duration::from_secs(300)).await;
        }else{
            debug_println!("connect sql failed, try agian after 5s");
            sleep(std::time::Duration::from_secs(5)).await;
        }
    }
}

//trim the record list to specified length with the specific time interval
async fn trim_record(max_num:i32,interval:u64,target:&'static str){
    let ds = create_data_store(true,false,false,false).await;
    let redis_data = ds.get::<RedisData>().unwrap();
    loop {
        let res = redis_data.llen(target).await;
        match res {
            Ok(num) => {
                debug_println!("{target} length is {}",num);
                let subtract = num - max_num;
                if subtract > 0 {
                    let _ = redis_data.lpop(target, subtract as usize).await;
                }
            },
            Err(err) => debug_println!("Flush record for {:?}",err),
        }
        sleep(std::time::Duration::from_secs(interval)).await;   
    }
}

//bussiness
pub async fn test(){
    let (flux_sender, _rx) = broadcast::channel::<DataTime>(3600);
    let (flux_vice_sender, _rx) = broadcast::channel::<DataTime>(3600);
    let (setpoint_sender, _rx) = broadcast::channel::<DataTime>(3600);
    let (setpoint_vice_sender, _rx) = broadcast::channel::<DataTime>(3600);
    let (switch_sender, _rx) = broadcast::channel::<DataTime>(3600);
    let (switch_vice_sender, _rx) = broadcast::channel::<DataTime>(3600);
    tokio::select! {
        _ = to_redis_str(setpoint_sender.subscribe(),"setpointStatus",) => { 
            std::process::exit(1);
        },
        _ = collect_data("setpoint",setpoint_sender) => { 
            std::process::exit(1);
        },
        _ = to_redis_str(setpoint_vice_sender.subscribe(),"setpointViceStatus",) => { 
            std::process::exit(1);
        },
        _ = collect_data("setpointVice",setpoint_vice_sender) => { 
            std::process::exit(1);
        },
        _ = to_redis_str(switch_sender.subscribe(),"switchStatus",) => { 
            std::process::exit(1);
        },
        _ = collect_data("switch",switch_sender) => { 
            std::process::exit(1);
        },
        _ = to_redis_str(switch_vice_sender.subscribe(),"switchViceStatus",) => { 
            std::process::exit(1);
        },
        _ = collect_data("switchVice",switch_vice_sender) => { 
            std::process::exit(1);
        },

        _ = trim_record(3600,600,"flux") => {
             std::process::exit(1); 
            },
        _ = flux_to_mysql(flux_sender.subscribe(),"flux",) => { 
            std::process::exit(1);
         },
        _ = to_redis_list(flux_sender.subscribe(),"flux",) => { 
            std::process::exit(1);
         },
        _ = collect_data("flux",flux_sender) => { 
            std::process::exit(1);
         },

        _ = trim_record(3600,600,"fluxVice") => { 
            std::process::exit(1);
         },
        _ = flux_to_mysql(flux_vice_sender.subscribe(),"fluxVice",) => { 
            std::process::exit(1);
         },
        _ = to_redis_list(flux_vice_sender.subscribe(),"fluxVice",) => { 
            std::process::exit(1);
         },
        _ = collect_data("fluxVice",flux_vice_sender) => { 
            std::process::exit(1);
         },

        _ = signal::ctrl_c() => { 
            std::process::exit(0);
         },
    };
}