use crate::store::app::DataStore;
use crate::store::opcuaSession;
use lazy_static::lazy_static;
use tokio::time::sleep;
use tokio::signal;
use tokio::task::{self,spawn_blocking};
use std::fmt::Display;
use std::str::FromStr;
use tokio::sync::broadcast::{self,Sender,Receiver};

use std::sync::Arc;
use mysql::prelude::Queryable;
use mysql::*;
use crate::store::{MysqlData::MysqlData,opcuaSession::*};
use AutoReagent::models::redis_data::RedisState as RedisData;
use crate::Models::Temperature::Temperature;
use crate::opcua_config::NodeConfig;
use crate::debug_println;

pub async fn transfer_data_to_plc<T>(target:&str,val:String) 
where T: 'static + Send + Sync + FromStr + Clone + Display + Copy,
{
    let ds = create_data_store(true,false,true,true).await;
    let config = ds.get::<NodeConfig>().unwrap();
    let redis_data = ds.get::<RedisData>().unwrap();
    let status_key = target.to_string() + "Status";
    lazy_static! { static ref mapper:DataStore = DataStore::new_variant_mapper(); }
        let id= config.node(target);
        let session_better = ds.get::<OpcuaSession>().unwrap();
        if let Ok(val)= val.parse::<T>(){
            let variant_func:Arc<_>  = mapper.get_func::<T>().unwrap();
            if OpcuaSession::async_write_single_retry(session_better, id, variant_func(val), 3).await
            {
                redis_data.setex_retry(&status_key, val,10,5).await;
            }
        }
}

pub async fn collect_data(target:&'static str,sender: Sender<DataTime>){
    let ds = create_data_store(false,false,true,true).await;
    let config = ds.get::<NodeConfig>().unwrap();
    loop {
        let session_better = ds.get::<OpcuaSession>().unwrap();
        let flux = config.node(target);
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

async fn create_data_store(redis:bool,mysql:bool,opcua_config:bool,opcua_session:bool,)->Arc<DataStore> {
    let ds = Arc::new(DataStore::new());
    if redis {
        let redis_data = RedisData::new("Iloveyouxuwu121234","redis://:Iloveyouxuwu121234@kazusa.vip", );
        ds.insert(redis_data);
    }
    if mysql {
        let mysql_data = MysqlData::new("mysql://root:121234@kazusa.vip:3000/plc").await;
        ds.insert(mysql_data);
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

pub async fn test(){
    let (flux_sender, _rx) = broadcast::channel::<DataTime>(3600);
    let (flux_vice_sender, _rx) = broadcast::channel::<DataTime>(3600);
    tokio::select! {
        // _ = transfer_data_to_plc::<bool>("switchVice") => { std::process::exit(1); },
        // _ = transfer_data_to_plc::<bool>("switch") => { std::process::exit(1); },
        // _ = transfer_data_to_plc::<f64>("setpoint") => { std::process::exit(1); },
        // _ = transfer_data_to_plc::<f64>("setpointVice") => { std::process::exit(1); },

        _ = trim_record(3600,600,"flux") => { std::process::exit(1); },
        _ = flux_to_mysql(flux_sender.subscribe(),"flux",) => { std::process::exit(1); },
        _ = flux_to_redis(flux_sender.subscribe(),"flux",) => { std::process::exit(1); },
        _ = collect_data("flux",flux_sender) => { std::process::exit(1); },

        _ = trim_record(3600,600,"fluxVice") => { std::process::exit(1); },
        _ = flux_to_mysql(flux_vice_sender.subscribe(),"fluxVice",) => { std::process::exit(1); },
        _ = flux_to_redis(flux_vice_sender.subscribe(),"fluxVice",) => { std::process::exit(1); },
        _ = collect_data("fluxVice",flux_vice_sender) => { std::process::exit(1); },

        _ = signal::ctrl_c() => { std::process::exit(0); },
    };
}

async fn flux_to_redis(mut recv: Receiver<DataTime>,target:&'static str) {
    let ds = create_data_store(true, false, false, false).await;
    let redis_data = ds.get::<RedisData>().unwrap();
    while let Ok(res) = recv.recv().await {
        redis_data.rpush_retry(target, vec![res.to_string()],3).await;
    }
}
async fn flux_to_mysql(mut recv: Receiver<DataTime>,table:&'static str) {
    let ds = create_data_store(false,true,false,false).await;
    loop {
        // let creat_cmd = format!("CREATE TABLE if not exists {table}(id bigint auto_increment,val double not null,time timestamp not null,primary key(id))");
        let insert_cmd = format!("INSERT INTO {table}(val,time) VALUES (:val, :time)");
        sleep(std::time::Duration::from_secs(20)).await;
        let mysql_data = ds.get::<MysqlData>().unwrap();
        if let Ok(mut conn) = spawn_blocking(move||{mysql_data.get_conn()}).await.unwrap(){
            let mut records:Vec<Temperature> = vec![];
            // let _ = task::spawn_blocking(move ||{conn.query_drop(&creat_cmd)}).await;
            while let Ok(msg) = recv.try_recv(){ records.push(Temperature::from(msg)); }
            match task::spawn_blocking( move || {
                conn.exec_batch(&insert_cmd,
                records.into_iter().map(|p| params! 
                    {
                        "val" => p.val,
                        "time" => p.time, 
                    }),
                )
            }).await.unwrap(){
                Ok(_) => debug_println!("Successfully store data to MySql"),
                Err(_) => debug_println!("Fail to store data to MySql"),
            };
            sleep(std::time::Duration::from_secs(300)).await;
        }else{
            debug_println!("connect sql failed, try agian after 5s");
            sleep(std::time::Duration::from_secs(3)).await;
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