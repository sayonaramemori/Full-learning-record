use std::sync::Arc;
use std::thread;
use mysql::prelude::Queryable;
use mysql::*;
use opcua::client::prelude::*;
use opcua::sync::*;
use crate::Models::ToRedis;
use crate::opcua_config::Operation::{create_session,read_batch, read_single, write_single};
use crate::store::{MysqlData::MysqlData,RedisData::RedisData};
use crate::Models::{MonitoredItemConfig::MonitorItemConfig,Temperature::Temperature};

pub fn subscribe_test(){
    // let config_ramp: MonitorItemConfig<_> = ("模拟器示例.函数.Ramp1".to_string(), to_redis_temp, 1000.0).into();
    let session = create_session("opc.tcp://127.0.0.1:49320");
    let config_temp: MonitorItemConfig<_>= ("ForOPCUA.PLC.temperature".to_string(), Temperature::to_redis, 1000.0).into();
    let redis_data = RedisData::new("redis://kazusa.vip/", "Iloveyouxuwu121234");
    let reference = Arc::new(redis_data);
    if 
    // subscribe_to_values(session.clone(),config_ramp,reference.clone()).is_ok() 
    // &&
    subscribe_to_values(session.clone(),config_temp,reference.clone()).is_ok()
    {
        to_mysql("mysql://root:121234@kazusa.vip:3000/plc".to_string(),"record".to_string(),reference.clone());
        let _ = Session::run(session);
    } else {
        println!("Error creating subscription");
    }
}


fn subscribe_to_values<T>(session: Arc<RwLock<Session>>,config: MonitorItemConfig<T>,redis_data:Arc<RedisData>) -> Result<u32, StatusCode> 
where T: Fn(&MonitoredItem,Arc<RedisData>) + Send + Sync + 'static
{
    let session = session.read();
    // let res = session.read(nodes_to_read, timestamps_to_return, max_age)
    // session.read(nodes_to_read, timestamps_to_return, max_age)
    
    let subscription_id = session.create_subscription(config.publish_interval, 10, 30, 0, 0, true, DataChangeCallback::new(move |changed_monitored_items| {
        changed_monitored_items.iter().for_each(|item| {
            (config.call_back)(item,redis_data.clone());
        });
    }))?;
    let items_to_create: Vec<MonitoredItemCreateRequest> = config.items.clone().into_iter()
        .map(|v| NodeId::new(2, v).into())
        .collect();
    let _ = session.create_monitored_items(subscription_id, TimestampsToReturn::Both, &items_to_create)?;
    Ok(subscription_id)
}


fn to_mysql(mysql_url:String,redis_key:String,redis_data:Arc<RedisData>) {
    thread::spawn(move ||{
        let mysql_data = MysqlData::new(&mysql_url);
        loop {
            let res = mysql_data.get_conn();
            if let Ok(mut conn) = res {
                if let Ok(res) = redis_data.as_ref().lpop(&redis_key, 2000){
                    let _ = conn.query_drop(
                    r"CREATE TABLE if not exists temp (
                        id bigint auto_increment,
                        val double not null,
                        time timestamp not null,
                        primary key(id)
                    )");
                    let res :Vec<Temperature>= res.into_iter().map(|s| Temperature::from(s)).collect();
                    // println!("res is {:?}",res);
                    let _ = conn.exec_batch(
                    r"INSERT INTO temp(val,time)
                        VALUES (:val, :time)",
                    res.iter().map(|p| params! {
                        "val" => p.val,
                        "time" => p.time,
                        })
                    );
                }
            }
            thread::sleep(std::time::Duration::from_secs(1800));
        }
    });
}