use crate::store::app::DataStore;
use lazy_static::lazy_static;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use mysql::prelude::Queryable;
// use mysql::prelude::*;
use mysql::*;
use opcua::client::prelude::*;
use opcua::sync::*;
use crate::opcua_config::Operation::{create_session,  read_single, write_single};
use crate::store::{MysqlData::MysqlData,RedisData::RedisData,opcuaSession::*};
use crate::Models::Temperature::Temperature;
use crate::opcua_config::NodeConfig;
use crate::debug_println;

//Monitoring the switch cmd in redis and write it to node when cmd comes.The same as setpoint
//When success, feedback to redis with suffix Status.
pub fn transfer_data_to_plc<T>(ds:Arc<DataStore>,target:&'static str) -> thread::JoinHandle<()>
where T: 'static + Send + Sync + FromStr + Clone + Display,
{
    let config = ds.get::<NodeConfig>().unwrap();
    let redis_data = ds.get::<RedisData>().unwrap();
    let session_better = ds.get::<OpcuaSession>().unwrap();
    let id= config.node(target);
    return thread::spawn(move ||{
        let status_key = target.to_string() + "Status";
        lazy_static! { static ref mapper:DataStore = DataStore::new_variant_mapper(); }
        loop {
            if let Ok(res) = redis_data.rpop(target, 1) {
                if !res.is_empty() {
                    if let Ok(val )= res[0].parse::<T>(){
                        let variant_func:Arc<_>  = mapper.get_func::<T>().unwrap();
                        if session_better.write_single_retry(&id, variant_func(val.clone()), 3){
                            redis_data.setex_retry(&status_key, val,10,5);
                        }
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

pub fn record_flux(ds:Arc<DataStore>,target:&'static str,sender:Sender<DataTime>)->thread::JoinHandle<()>{
    let config = ds.get::<NodeConfig>().unwrap();
    let redis_data = ds.get::<RedisData>().unwrap();
    let flux = config.node(target);
    let session_better = ds.get::<OpcuaSession>().unwrap();
    trim_record(redis_data.clone(), 3600,600,target);
    return thread::spawn(move ||{
        loop {
            let res = session_better.read_single(&flux);
            match res{
                Ok(res) => {
                    if let Some(res) = res  {
                        let _ = sender.send(res.clone());
                        redis_data.rpush_retry(target, res.to_string(),3);
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                },
                //wait for handling, session.write().is_connected();
                _ => debug_println!("Reading node failed"),
            };
        }
    });
}

fn create_data_store()->Arc<DataStore> {
    let redis_data = RedisData::new("redis://kazusa.vip/", "Iloveyouxuwu121234");
    let mysql_data = MysqlData::new("mysql://root:121234@kazusa.vip:3000/plc");
    let config = NodeConfig::new();
    let session = OpcuaSession::new("opc.tcp://127.0.0.1:49320");

    let ds = Arc::new(DataStore::new());
    ds.insert(redis_data);
    ds.insert(mysql_data);
    ds.insert(config);
    ds.insert(session);
    ds
}

fn record_data(ds:Arc<DataStore>,target:&'static str)->thread::JoinHandle<()>{
    let (sender,receiver) = std::sync::mpsc::channel::<DataTime>();
    flux_to_mysql(ds.clone(),receiver,target);
    return record_flux(ds,target,sender);
}

pub fn test(){
    let ds = create_data_store;
    transfer_data_to_plc::<bool>(ds(),"switch");
    transfer_data_to_plc::<bool>(ds(),"switchVice");
    transfer_data_to_plc::<f64>(ds(),"setpoint");
    transfer_data_to_plc::<f64>(ds(),"setpointVice");

    record_data(ds(), "flux");
    record_data(ds(), "fluxVice").join();
}

fn flux_to_mysql(ds:Arc<DataStore>,recv: Receiver<DataTime>,table:&'static str) {
    let mysql_data = ds.get::<MysqlData>().unwrap();
    thread::spawn(move ||{
        let creat_cmd = format!("CREATE TABLE if not exists {table}(id bigint auto_increment,val double not null,time timestamp not null,primary key(id))");
        let insert_cmd = format!("INSERT INTO {table}(val,time) VALUES (:val, :time)");
        loop {
            thread::sleep(std::time::Duration::from_secs(2));
            if let Ok(mut conn) = mysql_data.get_conn(){
                let mut records:Vec<Temperature> = vec![];
                let _ = conn.query_drop(&creat_cmd);
                while let Ok(msg) = recv.try_recv(){ records.push(Temperature::from(msg)); }
                match conn.exec_batch(&insert_cmd,
                    records.into_iter().map(|p| params! {
                    "val" => p.val,
                    "time" => p.time,
                })){
                    Ok(_) => debug_println!("Successfully store data to MySql"),
                    Err(_) => debug_println!("Fail to store data to MySql"),
                };
                thread::sleep(std::time::Duration::from_secs(300));
            }else{
                debug_println!("connect sql failed, try agian after 5s");
                thread::sleep(std::time::Duration::from_secs(3));
            }
        }
    });
}

//trim the record list to specified length with the specific time interval
fn trim_record(redis_data:Arc<RedisData>,max_num:i32,interval:u64,target:&'static str){
    thread::spawn(move ||{
        loop {
            let res = redis_data.llen(target);
            match res {
                Ok(num) => {
                    debug_println!("{target} length is {}",num);
                    let subtract = num - max_num;
                    if subtract > 0 {
                        let _ = redis_data.lpop(target, subtract as usize);
                    }
                },
                Err(err) => debug_println!("Flush record for {:?}",err),
            }
            std::thread::sleep(std::time::Duration::from_secs(interval));   
        }
    });
}